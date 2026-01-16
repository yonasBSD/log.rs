//! # Screen Logger
//!
//! A production-ready, cargo-style logging system for modern CLI applications.
//!
//! ## Overview
//!
//! This module provides a sophisticated yet ergonomic logging API that combines
//! human-friendly terminal output with powerful structured tracing. Built on
//! the tracing ecosystem, it scales from simple scripts to complex applications
//! without changing your logging calls.
//!
//! ## Features
//!
//! - **Cargo-Style Verbosity**: Four-level hierarchy (Quiet → Normal → Verbose → Trace)
//! - **Dual Output Modes**: Text for humans, JSON for machines and log aggregation
//! - **Automatic Span Management**: Track tasks and steps with zero boilerplate
//! - **Timing Built-In**: Automatically measure and report task duration
//! - **Global Singleton**: Call `log().ok("message")` from anywhere—no passing loggers around
//! - **Smart Suppression**: Quiet mode respects errors while hiding noise
//! - **Multiple Formatters**: `SimpleLogger` for basics, `ModernLogger` for eye candy
//! - **Tracing Integration**: Seamless structured logging when you need observability
//!
//! ## Quick Start
//!
//! ```rust
//! use log_rs::logging::{
//!     set_logger, log, Printer, SimpleLogger, Verbosity, LogFormat, SimpleBackend,
//! };
//!
//! // Initialize once at startup
//! let logger = Printer::new(SimpleLogger, SimpleBackend, LogFormat::Text, Verbosity::Normal);
//! set_logger(logger);
//!
//! // Use anywhere in your app
//! log::intro("Deploying application");
//! log::step("Building assets");
//! log::step("Uploading files");
//! log::ok("All files uploaded");
//! log::outro("Deployment complete");
//! // → Outputs:
//! // → Deploying application
//! // ⠿ Building assets
//! // ⠿ Uploading files
//! // ✔ All files uploaded
//! // ✔ Deployment complete (took 2.3s)
//! ```
//!
//! ## Verbosity Levels Explained
//!
//! - **Quiet** (`-q`): Errors only—perfect for cron jobs and CI
//! - **Normal** (default): Standard CLI output with success/warning/info
//! - **Verbose** (`-v`): Adds debug logs and tracing spans for troubleshooting
//! - **Trace** (`-vv`): Full diagnostic output—see everything the app does
//!
//! ## Output Formats
//!
//! **Text Mode** (default):
//! ```text
//! ✔ Server started
//! ⠿ Processing request
//! ⚠ Cache miss for key: user_123
//! ```
//!
//! **JSON Mode** (--format=json):
//! ```json
//! {"level":"info","message":"✔ Server started","timestamp":"2026-01-15T10:30:00Z"}
//! {"level":"info","message":"⠿ Processing request","timestamp":"2026-01-15T10:30:01Z"}
//! {"level":"warn","message":"⚠ Cache miss","timestamp":"2026-01-15T10:30:02Z"}
//! ```
//!
//! ## Architecture
//!
//! Two-layer design for clean separation of concerns:
//!
//! 1. **`FormatLogger` Trait**: Formats messages into strings (`SimpleLogger`, `ModernLogger`)
//!    - Handles quiet-mode suppression
//!    - Returns styled strings ready for display
//!
//! 2. **`ScreenLogger` Trait**: Prints formatted messages to terminal
//!    - `Printer<L, B>` implementation adds span management
//!    - Routes output to stdout/stderr or JSON
//!    - Integrates with tracing for structured logs
//!
//! A third layer, **`RenderBackend`**, controls *how* formatted strings are rendered:
//!   - `SimpleBackend` → `println!` / `eprintln!`
//!   - `ModernBackend` → `cliclack`-style rich output
//!
//! This separation makes it trivial to:
//! - Add new formatters (Markdown, HTML, etc.)
//! - Add new backends (TUI, GUI, remote logging)
//! - Test formatting logic without I/O
//! - Swap output backends without changing user code
//!
//! ## Design Philosophy
//!
//! Great logging is invisible until you need it. This system prioritizes:
//!
//! - **Ergonomics**: `log().ok("done")` beats passing logger instances everywhere
//! - **Clarity**: Visual symbols (✔ ⚠ ✗) communicate status at a glance
//! - **Performance**: Lazy formatting, zero-cost quiet mode, minimal allocations
//! - **Flexibility**: Start simple, add structure as your app grows
//! - **Professionalism**: Output that looks polished in terminals and log viewers
//!
//! Whether you're building a quick script or a production service, this logger
//! adapts to your needs without getting in your way.

pub mod log;

pub static L: LogProxy = LogProxy;

/// Proxy value so callers can write `L.ok("msg")` or `log().ok("msg")`.
pub struct LogProxy;

impl LogProxy {
    pub fn ok(&self, msg: &str) {
        log().ok(msg);
    }

    pub fn warn(&self, msg: &str) {
        log().warn(msg);
    }

    pub fn err(&self, msg: &str) {
        log().err(msg);
    }

    pub fn info(&self, msg: &str) {
        log().info(msg);
    }

    pub fn dim(&self, msg: &str) {
        log().dim(msg);
    }

    pub fn intro(&self, msg: &str) {
        log().intro(msg);
    }

    pub fn outro(&self, msg: &str) {
        log().outro(msg);
    }

    pub fn done(&self, msg: &str) {
        log().done(msg);
    }

    pub fn step(&self, msg: &str) {
        log().step(msg);
    }

    pub fn debug(&self, msg: &str) {
        log().debug(msg);
    }

    pub fn trace(&self, msg: &str) {
        log().trace(msg);
    }
}

use crate::config;
use once_cell::sync::OnceCell;
use std::{sync::Arc, sync::Mutex, time::Instant};
use terminal_banner::Banner;
use tracing::{Level, debug, error, info, span, span::Span, trace, warn};
use tracing_subscriber::{Layer, Registry, fmt::writer::BoxMakeWriter, prelude::*};

const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
const PROJECT_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

/// A global, thread-safe screen logger.
///
/// Applications call `log().ok("message")` without needing to pass
/// logger instances around. This keeps the API clean and ergonomic.
///
/// The logger must be initialized once at startup using `set_logger()`.
static LOGGER: OnceCell<Arc<dyn ScreenLogger + Send + Sync>> = OnceCell::new();

/// One-time guard for tracing subscriber initialization.
///
/// We keep tracing global and idempotent, while allowing per-Printer
/// verbosity to control `RUST_LOG` before the first init.
static INIT: OnceCell<()> = OnceCell::new();

/// Set the global logger.
///
/// This should be called once during program initialization.
/// Subsequent calls are ignored.
pub fn set_logger<L: ScreenLogger + Send + Sync + 'static>(logger: L) {
    let _ = LOGGER.set(Arc::new(logger));
}

/// Retrieve the global logger.
///
/// Panics if the logger has not been initialized.
/// Applications should call `set_logger()` early in `main()`.
fn log() -> &'static Arc<dyn ScreenLogger + Send + Sync> {
    LOGGER.get().expect("Logger not initialized")
}

/// Initialize the global tracing subscriber.
///
/// This is idempotent: only the first call wins. Subsequent calls
/// are cheap no-ops. The effective filter is taken from the current
/// `RUST_LOG` environment variable at the time of the first call.
use tracing_subscriber::filter::LevelFilter;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    if INIT.get().is_some() {
        return Ok(()); // already initialized
    }

    INIT.set(()).ok();
    env_rs::init()?;

    let telemetry_fmt = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .without_time()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_writer(BoxMakeWriter::new(std::io::stderr));

    // Always enable all levels; our own verbosity controls what we actually emit.
    let registry = Registry::default().with(telemetry_fmt.with_filter(LevelFilter::TRACE));

    #[cfg(feature = "tokio-console")]
    let registry = registry.with(console_subscriber::spawn());

    tracing::subscriber::set_global_default(registry)?;

    tracing::debug!("Logging initialized!");
    tracing::trace!("Tracing initialized!");
    tracing::debug!("Ready to begin...");

    if std::env::var("RUST_LOG").is_ok()
        && ["debug", "trace"].contains(&std::env::var("RUST_LOG").unwrap().to_lowercase().as_str())
    {
        let banner = Banner::new()
            .text(format!("Welcome to {PROJECT_NAME}!").into())
            .text(PROJECT_DESC.into())
            .render();

        println!("{banner}");
    }

    Ok(())
}

fn format_duration(d: std::time::Duration) -> String {
    if d.as_secs() > 0 {
        format!("{:.1}s", d.as_secs_f64())
    } else {
        format!("{}ms", d.as_millis())
    }
}

/// Cargo-style verbosity levels.
///
/// Quiet   → suppress almost all output
/// Normal  → standard CLI output (println)
/// Verbose → use tracing spans + debug-level logs
/// Trace   → full tracing, including trace-level logs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,   // -q
    Normal,  // default
    Verbose, // -v
    Trace,   // -vv
}

/// Output format for the logger.
///
/// Text → human‑friendly CLI output (println / eprintln)
/// Json → machine‑friendly structured output (one JSON object per event)
///
/// JSON mode is ideal for CI logs, log aggregation, or tools that parse output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Text,
    Json,
}

/// A span that tracks when it was entered so we can compute
/// how long the task took when `outro()` / `done()` is called.
struct TimedSpan {
    /// The active tracing span (dropped to exit)
    span: Span,

    /// Timestamp when the span was entered
    start: Instant,
}

/// A logger that *only formats* messages into strings.
///
/// This trait provides:
///   - A set of `_raw` methods that concrete loggers must implement.
///   - Default high-level methods (`ok`, `warn`, etc.) that:
///         • automatically apply quiet-mode suppression
///         • wrap the raw formatting
///
/// The goal: avoid code duplication across loggers while keeping
/// formatting and quiet-mode logic cleanly separated.
pub trait FormatLogger {
    /// Returns true if output should be suppressed.
    /// Override if a logger wants custom quiet-mode behavior.
    fn is_quiet(&self) -> bool {
        config::isquiet()
    }

    /// Verbose mode flag (for debug/trace)
    fn is_verbose(&self) -> bool {
        config::isverbose()
    }

    // ---------------------------------------------------------------------
    // RAW METHODS (must be implemented by each logger)
    // These return *unconditional* formatted strings with no quiet-mode logic.
    // ---------------------------------------------------------------------

    /// Format a success message (e.g., green checkmark)
    fn ok_raw(&self, m: &str) -> String;

    /// Format a warning message (e.g., yellow warning sign)
    fn warn_raw(&self, m: &str) -> String;

    /// Format an error message (always shown)
    fn err_raw(&self, m: &str) -> String;

    /// Format an informational message
    fn info_raw(&self, m: &str) -> String;

    /// Format a dim/muted message
    fn dim_raw(&self, m: &str) -> String;

    /// Format an intro message (start of a task)
    fn intro_raw(&self, m: &str) -> String;

    /// Format an outro message (end of a task)
    fn outro_raw(&self, m: &str) -> String;

    /// Format a done message (end of a task)
    fn done_raw(&self, m: &str) -> String;

    /// Format a step/progress message
    fn step_raw(&self, m: &str) -> String;

    /// Debug-level message (verbose only)
    fn debug_raw(&self, m: &str) -> String;

    /// Trace-level message (verbose only)
    fn trace_raw(&self, m: &str) -> String;

    // ---------------------------------------------------------------------
    // DEFAULT METHODS (quiet-mode aware)
    // These wrap the raw methods and apply quiet-mode suppression.
    // ---------------------------------------------------------------------

    /// Success message (suppressed in quiet mode)
    fn ok(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.ok_raw(m))
        }
    }

    /// Warning message (suppressed in quiet mode)
    fn warn(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.warn_raw(m))
        }
    }

    /// Error message (never suppressed)
    fn err(&self, m: &str) -> String {
        self.err_raw(m)
    }

    /// Info message (suppressed in quiet mode)
    fn info(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.info_raw(m))
        }
    }

    /// Dim/muted message (suppressed in quiet mode)
    fn dim(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.dim_raw(m))
        }
    }

    /// Intro message (suppressed in quiet mode)
    fn intro(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.intro_raw(m))
        }
    }

    /// Outro message (suppressed in quiet mode)
    fn outro(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.outro_raw(m))
        }
    }

    /// Done message (suppressed in quiet mode)
    fn done(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.done_raw(m))
        }
    }

    /// Step/progress message (suppressed in quiet mode)
    fn step(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.step_raw(m))
        }
    }

    /// Debug messages only appear when verbose mode is enabled.
    fn debug(&self, m: &str) -> Option<String> {
        if self.is_verbose() {
            Some(self.debug_raw(m))
        } else {
            None
        }
    }

    /// Trace messages only appear when verbose mode is enabled.
    fn trace(&self, m: &str) -> Option<String> {
        if self.is_verbose() {
            Some(self.trace_raw(m))
        } else {
            None
        }
    }
}

/// A simple ANSI-based logger that formats messages using
/// plain ASCII or ANSI escape codes depending on configuration.
///
/// This logger does *not* print anything — it only formats strings.
/// Quiet-mode suppression is handled by the `FormatLogger` trait.
pub struct SimpleLogger;

impl FormatLogger for SimpleLogger {
    fn ok_raw(&self, m: &str) -> String {
        if config::isnocolor() {
            format!("+ {m}")
        } else {
            format!("\x1b[32m✔\x1b[0m {m}")
        }
    }

    fn warn_raw(&self, m: &str) -> String {
        if config::isnocolor() {
            format!("! {m}")
        } else {
            format!("\x1b[33m⚠\x1b[0m {m}")
        }
    }

    fn err_raw(&self, m: &str) -> String {
        if config::isnocolor() {
            format!("X {m}")
        } else {
            format!("\x1b[31m✗\x1b[0m {m}")
        }
    }

    fn info_raw(&self, m: &str) -> String {
        format!("  {m}")
    }

    fn dim_raw(&self, m: &str) -> String {
        if config::isnocolor() {
            format!("  {m}")
        } else {
            format!("\x1b[90m  {m}\x1b[0m")
        }
    }

    fn intro_raw(&self, m: &str) -> String {
        format!("→ {m}")
    }

    fn outro_raw(&self, m: &str) -> String {
        format!("✓ {m}")
    }

    fn done_raw(&self, _m: &str) -> String {
        "✓ Done!".to_string()
    }

    fn step_raw(&self, m: &str) -> String {
        if config::isnocolor() {
            format!("* {m}")
        } else {
            format!("\x1b[36m⠿\x1b[0m {m}")
        }
    }

    fn debug_raw(&self, m: &str) -> String {
        if config::isnocolor() {
            format!("[debug] {m}")
        } else {
            format!("\x1b[34m[debug]\x1b[0m {m}")
        }
    }

    fn trace_raw(&self, m: &str) -> String {
        if config::isnocolor() {
            format!("[trace] {m}")
        } else {
            format!("\x1b[90m[trace]\x1b[0m {m}")
        }
    }
}

/// A modern, minimal logger inspired by cliclack's visual style.
///
/// This logger only *formats* messages — it does not print.
/// Quiet-mode suppression is handled by the `FormatLogger` trait.
pub struct ModernLogger;

impl FormatLogger for ModernLogger {
    fn ok_raw(&self, m: &str) -> String {
        format!("✔ {m}")
    }

    fn warn_raw(&self, m: &str) -> String {
        format!("⚠ {m}")
    }

    fn err_raw(&self, m: &str) -> String {
        format!("✗ {m}")
    }

    fn info_raw(&self, m: &str) -> String {
        format!("ℹ {m}")
    }

    fn dim_raw(&self, m: &str) -> String {
        format!("› {m}")
    }

    fn intro_raw(&self, m: &str) -> String {
        format!("→ {m}")
    }

    fn outro_raw(&self, m: &str) -> String {
        format!("✔ {m}")
    }

    fn done_raw(&self, m: &str) -> String {
        format!("✔ {m}")
    }

    fn step_raw(&self, m: &str) -> String {
        format!("⠿ {m}")
    }

    fn debug_raw(&self, m: &str) -> String {
        format!("🔍 {m}")
    }

    fn trace_raw(&self, m: &str) -> String {
        format!("📡 {m}")
    }
}

/// A backend that knows how to *render* formatted strings.
///
/// This is where we map high-level semantics to concrete output:
///   - `SimpleBackend` → println!/eprintln!
///   - `ModernBackend` → cliclack::intro/outro/step/etc.
pub trait RenderBackend {
    fn render_error(&self, msg: &str) -> anyhow::Result<()>;
    fn render_info(&self, msg: &str) -> anyhow::Result<()>;
    fn render_remark(&self, msg: &str) -> anyhow::Result<()>;
    fn render_step(&self, msg: &str) -> anyhow::Result<()>;
    fn render_success(&self, msg: &str) -> anyhow::Result<()>;
    fn render_warning(&self, msg: &str) -> anyhow::Result<()>;
    fn render_intro(&self, msg: &str) -> anyhow::Result<()>;
    fn render_outro(&self, msg: &str) -> anyhow::Result<()>;
}

/// A simple backend that renders to stdout/stderr using println!/eprintln!.
pub struct SimpleBackend;

impl RenderBackend for SimpleBackend {
    fn render_error(&self, msg: &str) -> anyhow::Result<()> {
        eprintln!("{msg}");
        Ok(())
    }

    fn render_info(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_remark(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_step(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_success(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_warning(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_intro(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_outro(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }
}

/// A backend that renders using cliclack's rich CLI primitives.
///
/// Mapping (your design):
///   error  → error
///   info   → info
///   dim    → remark
///   step   → step
///   ok     → success
///   warn   → warning
///   intro  → intro
///   outro  → outro
///   done   → outro (handled in Printer)
///   debug  → info (handled in Printer)
///   trace  → info (handled in Printer)
pub struct ModernBackend;

impl RenderBackend for ModernBackend {
    fn render_error(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::error(msg)?;
        Ok(())
    }

    fn render_info(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::info(msg)?;
        Ok(())
    }

    fn render_remark(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::remark(msg)?;
        Ok(())
    }

    fn render_step(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::step(msg)?;
        Ok(())
    }

    fn render_success(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::success(msg)?;
        Ok(())
    }

    fn render_warning(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::warning(msg)?;
        Ok(())
    }

    fn render_intro(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::intro(msg)?;
        Ok(())
    }

    fn render_outro(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::outro(msg)?;
        Ok(())
    }
}

/// A logger that *prints* messages to the screen.
///
/// This trait represents the high-level logging API that most users interact with.
/// It does not format messages itself — instead, it delegates to a `FormatLogger`
/// implementation and then prints the resulting strings via a `RenderBackend`.
pub trait ScreenLogger {
    /// Print a success message
    fn ok(&self, m: &str);

    /// Print a warning message
    fn warn(&self, m: &str);

    /// Print an error message (never suppressed)
    fn err(&self, m: &str);

    /// Print an informational message
    fn info(&self, m: &str);

    /// Print a dim/muted message
    fn dim(&self, m: &str);

    /// Print an intro message (start of a task)
    fn intro(&self, m: &str);

    /// Print an outro message (end of a task)
    fn outro(&self, m: &str);

    /// Print a done message (end of a task)
    fn done(&self, m: &str);

    /// Print a step/progress message
    fn step(&self, m: &str);

    /// Verbose-only debug message
    fn debug(&self, m: &str);

    /// Verbose-only trace message
    fn trace(&self, m: &str);
}

/// A screen logger that prints formatted messages and, in verbose/trace mode,
/// also emits structured tracing spans.
///
/// This logger supports:
///   - Nested task spans via `intro()` / `outro()` / `done()`
///   - Nested step spans inside tasks
///   - Timing of task spans
///   - Cargo-style verbosity levels
///
/// It is generic over:
///   - `L: FormatLogger`  → how messages are formatted
///   - `B: RenderBackend` → how formatted strings are rendered
pub struct Printer<L: FormatLogger, B: RenderBackend> {
    inner: L,
    backend: B,
    tasks: Mutex<Vec<TimedSpan>>,
    steps: Mutex<Vec<Span>>,
    format: LogFormat,
    verbosity: Verbosity,
}

impl<L: FormatLogger, B: RenderBackend> Printer<L, B> {
    /// Create a new printer with the given formatter, backend, and output format.
    ///
    /// This also:
    ///   - Sets `RUST_LOG` based on the requested verbosity
    ///   - Initializes the global tracing subscriber once
    pub fn new(inner: L, backend: B, format: LogFormat, verbosity: Verbosity) -> Self {
        // Set RUST_LOG automatically based on verbosity.
        //
        // This must happen *before* the first call to `init()` so that
        // `EnvFilter::from_default_env()` sees the right value. Subsequent
        // calls to `Printer::new()` will not reconfigure tracing, but they
        // still update the in-process config flags used by FormatLogger.
        match verbosity {
            Verbosity::Quiet => {
                crate::config::setquiet(true);
                crate::config::setverbose(false);
            }
            Verbosity::Normal => {
                crate::config::setquiet(false);
                crate::config::setverbose(false);
            }
            Verbosity::Verbose | Verbosity::Trace => {
                crate::config::setquiet(false);
                crate::config::setverbose(true);
            }
        }

        // Initialize tracing (idempotent)
        let _ = crate::logging::init();

        Self {
            inner,
            backend,
            tasks: Mutex::new(Vec::new()),
            steps: Mutex::new(Vec::new()),
            format,
            verbosity,
        }
    }

    /// Emit a single log event as a JSON object.
    ///
    /// This function:
    ///   - Builds a structured JSON object using `serde_json`
    ///   - Includes a timestamp for easier log correlation
    ///   - Sends errors to stderr and all other levels to stdout
    ///   - Does NOT create tracing spans (that happens elsewhere)
    #[allow(clippy::unused_self)]
    fn emit_json(&self, level: &str, message: &str) {
        let obj = serde_json::json!({
            "level": level,
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        match level {
            "error" => eprintln!("{obj}"),
            _ => println!("{obj}"),
        }
    }
}

impl<L: FormatLogger, B: RenderBackend> ScreenLogger for Printer<L, B> {
    fn intro(&self, m: &str) {
        if let Some(s) = self.inner.intro(m) {
            match self.format {
                LogFormat::Json => {
                    self.emit_json("info", &s);
                }
                LogFormat::Text => {
                    // intro → intro
                    let _ = self.backend.render_intro(&s);

                    if self.inner.is_verbose() {
                        let sp = span!(Level::INFO, "task", message = %m);
                        self.tasks.lock().unwrap().push(TimedSpan {
                            span: sp,
                            start: Instant::now(),
                        });
                        info!("{s}");
                    }
                }
            }
        }
    }

    fn outro(&self, m: &str) {
        if let Some(s) = self.inner.outro(m) {
            match self.format {
                LogFormat::Json => self.emit_json("info", &s),
                LogFormat::Text => {
                    if self.inner.is_verbose() {
                        // Close step spans
                        self.steps.lock().unwrap().clear();

                        // Close task span
                        let task = self.tasks.lock().unwrap().pop();
                        if let Some(TimedSpan { span, start }) = task {
                            drop(span);
                            let elapsed = start.elapsed();
                            let timing = format_duration(elapsed);
                            let msg = format!("{s} (took {timing})");

                            // outro → outro
                            let _ = self.backend.render_outro(&msg);
                            info!("{msg}");
                        }
                    } else {
                        let _ = self.backend.render_outro(&s);
                    }
                }
            }
        }
    }

    fn done(&self, m: &str) {
        if let Some(s) = self.inner.done(m) {
            match self.format {
                LogFormat::Json => self.emit_json("info", &s),
                LogFormat::Text => {
                    if self.inner.is_verbose() {
                        self.steps.lock().unwrap().clear();

                        let task = self.tasks.lock().unwrap().pop();
                        if let Some(TimedSpan { span, start }) = task {
                            drop(span);
                            let elapsed = start.elapsed();
                            let timing = format_duration(elapsed);
                            let msg = format!("{s} (took {timing})");

                            // done → outro
                            let _ = self.backend.render_outro(&msg);
                            info!("{msg}");
                        }
                    } else {
                        // done → outro
                        let _ = self.backend.render_outro(&s);
                    }
                }
            }
        }
    }

    fn step(&self, m: &str) {
        if let Some(s) = self.inner.step(m) {
            match self.format {
                LogFormat::Json => {
                    self.emit_json("info", &s);
                }
                LogFormat::Text => {
                    // step → step
                    let _ = self.backend.render_step(&s);

                    if self.inner.is_verbose() {
                        let sp = span!(Level::INFO, "step", message = %m);
                        self.steps.lock().unwrap().push(sp);
                        info!("{s}");
                    }
                }
            }
        }
    }

    fn ok(&self, m: &str) {
        if let Some(s) = self.inner.ok(m) {
            match self.format {
                LogFormat::Json => self.emit_json("info", &s),
                LogFormat::Text => {
                    // ok → success
                    let _ = self.backend.render_success(&s);
                }
            }
        }
    }

    fn warn(&self, m: &str) {
        if let Some(s) = self.inner.warn(m) {
            match self.format {
                LogFormat::Json => self.emit_json("warn", &s),
                LogFormat::Text => {
                    // warn → warning
                    let _ = self.backend.render_warning(&s);
                    warn!("{s}");
                }
            }
        }
    }

    fn err(&self, m: &str) {
        let s = self.inner.err(m);

        match self.format {
            LogFormat::Json => self.emit_json("error", &s),
            LogFormat::Text => {
                // error → error
                let _ = self.backend.render_error(&s);
                error!("{s}");
            }
        }
    }

    fn info(&self, m: &str) {
        if let Some(s) = self.inner.info(m) {
            match self.format {
                LogFormat::Json => self.emit_json("info", &s),
                LogFormat::Text => {
                    // info → info
                    let _ = self.backend.render_info(&s);
                }
            }
        }
    }

    fn dim(&self, m: &str) {
        if let Some(s) = self.inner.dim(m) {
            match self.format {
                LogFormat::Json => self.emit_json("debug", &s),
                LogFormat::Text => {
                    // dim → remark
                    let _ = self.backend.render_remark(&s);
                }
            }
        }
    }

    fn debug(&self, m: &str) {
        if let Some(s) = self.inner.debug(m) {
            match self.format {
                LogFormat::Json => self.emit_json("debug", &s),
                LogFormat::Text => {
                    // debug → info (screen) + debug! (tracing)
                    //let _ = self.backend.render_info(&s);
                    debug!("{s}");
                }
            }
        }
    }

    fn trace(&self, m: &str) {
        // Respect trace mode via FormatLogger and explicit Verbosity::Trace.
        //
        // This ensures:
        //   - `trace!()` calls only appear when the user explicitly requested
        //     trace verbosity (e.g., `-vv`).
        //   - Tests that expect trace-only visibility can rely on Verbosity.
        if let Some(s) = self.inner.trace(m)
            && self.verbosity == Verbosity::Trace
        {
            match self.format {
                LogFormat::Json => self.emit_json("trace", &s),
                LogFormat::Text => {
                    // trace → tracing::trace! only (no extra stdout noise)
                    //let _ = self.backend.render_info(&s);
                    trace!("{s}");
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[cfg(test)]
#[path = "behavior_tests.rs"]
mod behavior_tests;
