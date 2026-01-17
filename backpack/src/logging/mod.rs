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
//! - **Structured Fields**: Attach key/value pairs to logs in JSON mode
//! - **Task Tree Introspection**: Dump active tasks and timings in verbose/trace mode
//! - **Progress API**: Lightweight progress handle for long-running tasks
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

    pub fn done(&self) {
        log().done();
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

    /// Dump the current task tree (verbose/trace only).
    pub fn dump_tree(&self) {
        log().dump_tree();
    }

    /// Start a progress handle for a long-running task.
    pub fn progress(&self, msg: &str) -> Progress {
        Progress::new(msg)
    }
}

use crate::config;
use once_cell::sync::OnceCell;
use std::{collections::BTreeMap, sync::Arc, sync::Mutex, time::Instant};
use terminal_banner::Banner;
use tracing::{Level, debug, error, info, span, span::Span, trace, warn};
use tracing_subscriber::{
    Layer, Registry, filter::LevelFilter, fmt::writer::BoxMakeWriter, prelude::*,
};

const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
const PROJECT_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

/// A global, thread-safe screen logger.
static LOGGER: OnceCell<Arc<dyn ScreenLogger + Send + Sync>> = OnceCell::new();

/// One-time guard for tracing subscriber initialization.
static INIT: OnceCell<()> = OnceCell::new();

/// Set the global logger.
pub fn set_logger<L: ScreenLogger + Send + Sync + 'static>(logger: L) {
    let _ = LOGGER.set(Arc::new(logger));
}

/// Retrieve the global logger.
fn log() -> &'static Arc<dyn ScreenLogger + Send + Sync> {
    LOGGER.get().expect("Logger not initialized")
}

/// Initialize the global tracing subscriber.
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    if INIT.get().is_some() {
        return Ok(());
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

    let registry = Registry::default().with(telemetry_fmt.with_filter(LevelFilter::TRACE));

    #[cfg(feature = "tokio-console")]
    let registry = registry.with(console_subscriber::spawn());

    tracing::subscriber::set_global_default(registry)?;

    if std::env::var("RUST_LOG").is_ok()
        && ["debug", "trace"].contains(&std::env::var("RUST_LOG").unwrap().to_lowercase().as_str())
    {
        let banner = Banner::new()
            .text(format!("Welcome to {PROJECT_NAME}!\n").into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,   // -q
    Normal,  // default
    Verbose, // -v
    Trace,   // -vv
}

/// Output format for the logger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Text,
    Json,
}

/// Structured fields attached to a log event.
pub type Fields = BTreeMap<String, String>;

/// A span that tracks when it was entered so we can compute
/// how long the task took when `outro()` / `done()` is called.
#[derive(Debug)]
struct TimedSpan {
    span: Span,
    start: Instant,
    label: String,
}

/// A logger that *only formats* messages into strings.
pub trait FormatLogger {
    fn is_quiet(&self) -> bool {
        config::isquiet()
    }

    fn is_verbose(&self) -> bool {
        config::isverbose()
    }

    fn ok_raw(&self, m: &str) -> String;
    fn warn_raw(&self, m: &str) -> String;
    fn err_raw(&self, m: &str) -> String;
    fn info_raw(&self, m: &str) -> String;
    fn dim_raw(&self, m: &str) -> String;
    fn intro_raw(&self, m: &str) -> String;
    fn outro_raw(&self, m: &str) -> String;
    fn done_raw(&self) -> String;
    fn step_raw(&self, m: &str) -> String;
    fn debug_raw(&self, m: &str) -> String;
    fn trace_raw(&self, m: &str) -> String;

    fn ok(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.ok_raw(m))
        }
    }

    fn warn(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.warn_raw(m))
        }
    }

    fn err(&self, m: &str) -> String {
        self.err_raw(m)
    }

    fn info(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.info_raw(m))
        }
    }

    fn dim(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.dim_raw(m))
        }
    }

    fn intro(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.intro_raw(m))
        }
    }

    /// Outro is *not* suppressed in quiet mode so that quiet builds/tests
    /// can still show timing summaries.
    fn outro(&self, m: &str) -> Option<String> {
        Some(self.outro_raw(m))
    }

    /// Done is *not* suppressed in quiet mode for the same reason as `outro`.
    fn done(&self) -> Option<String> {
        Some(self.done_raw())
    }

    fn step(&self, m: &str) -> Option<String> {
        if self.is_quiet() {
            None
        } else {
            Some(self.step_raw(m))
        }
    }

    fn debug(&self, m: &str) -> Option<String> {
        if self.is_verbose() {
            Some(self.debug_raw(m))
        } else {
            None
        }
    }

    fn trace(&self, m: &str) -> Option<String> {
        if self.is_verbose() {
            Some(self.trace_raw(m))
        } else {
            None
        }
    }
}

/// A simple ANSI-based logger.
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

    fn done_raw(&self) -> String {
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

/// A modern, minimal logger inspired by cliclack.
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

    fn done_raw(&self) -> String {
        format!("✔ Done!")
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
pub trait RenderBackend {
    fn render_error(&self, msg: &str) -> anyhow::Result<()>;
    fn render_info(&self, msg: &str) -> anyhow::Result<()>;
    fn render_remark(&self, msg: &str) -> anyhow::Result<()>;
    fn render_step(&self, msg: &str) -> anyhow::Result<()>;
    fn render_success(&self, msg: &str) -> anyhow::Result<()>;
    fn render_warning(&self, msg: &str) -> anyhow::Result<()>;
    fn render_intro(&self, msg: &str) -> anyhow::Result<()>;
    fn render_outro(&self, msg: &str) -> anyhow::Result<()>;
    fn render_debug(&self, msg: &str) -> anyhow::Result<()>;
    fn render_trace(&self, msg: &str) -> anyhow::Result<()>;
}

/// A simple backend that renders to stdout/stderr.
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

    fn render_debug(&self, msg: &str) -> anyhow::Result<()> {
        eprintln!("{msg}");
        Ok(())
    }

    fn render_trace(&self, msg: &str) -> anyhow::Result<()> {
        eprintln!("{msg}");
        Ok(())
    }
}

/// A backend that renders using cliclack's rich CLI primitives.
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

    fn render_debug(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::remark(msg)?;
        Ok(())
    }

    fn render_trace(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::remark(msg)?;
        Ok(())
    }
}

/// High-level logging API.
pub trait ScreenLogger {
    fn ok(&self, m: &str);
    fn warn(&self, m: &str);
    fn err(&self, m: &str);
    fn info(&self, m: &str);
    fn dim(&self, m: &str);
    fn intro(&self, m: &str);
    fn outro(&self, m: &str);
    fn done(&self);
    fn step(&self, m: &str);
    fn debug(&self, m: &str);
    fn trace(&self, m: &str);
    /// Dump the current task tree (verbose/trace only).
    fn dump_tree(&self);
}

/// A lightweight progress handle.
///
/// This is intentionally simple: it just emits step/info/done messages
/// through the global logger, so it works with any backend.
pub struct Progress {
    label: String,
    current: u64,
    total: Option<u64>,
}

impl Progress {
    pub fn new(label: &str) -> Self {
        log().intro(label);
        Self {
            label: label.to_string(),
            current: 0,
            total: None,
        }
    }

    pub fn with_total(label: &str, total: u64) -> Self {
        log().intro(label);
        Self {
            label: label.to_string(),
            current: 0,
            total: Some(total),
        }
    }

    pub fn update(&mut self, current: u64, total: u64) {
        self.current = current;
        self.total = Some(total);
        let msg = format!("{}: {}/{}", self.label, self.current, total);
        log().step(&msg);
    }

    pub fn tick(&mut self) {
        self.current += 1;
        if let Some(total) = self.total {
            let msg = format!("{}: {}/{}", self.label, self.current, total);
            log().step(&msg);
        } else {
            let msg = format!("{}: {}", self.label, self.current);
            log().step(&msg);
        }
    }

    pub fn finish(self, _msg: &str) {
        log().done();
    }
}

/// A screen logger that prints formatted messages and, in verbose/trace mode,
/// also emits structured tracing spans.
pub struct Printer<L: FormatLogger, B: RenderBackend> {
    inner: L,
    backend: B,
    tasks: Mutex<Vec<TimedSpan>>,
    steps: Mutex<Vec<Span>>,
    format: LogFormat,
    verbosity: Verbosity,
}

impl<L: FormatLogger, B: RenderBackend> Printer<L, B> {
    pub fn new(inner: L, backend: B, format: LogFormat, verbosity: Verbosity) -> Self {
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
}

impl<L: FormatLogger, B: RenderBackend> ScreenLogger for Printer<L, B> {
    fn intro(&self, m: &str) {
        if let Some(s) = self.inner.intro(m) {
            match self.format {
                LogFormat::Json => {
                    self.emit_json(LogLevel::Info, &s);
                }
                LogFormat::Text => {
                    let _ = self.backend.render_intro(&s);
                    if self.inner.is_verbose() {
                        info!("{s}");
                    }
                }
            }
        }

        let sp = span!(Level::INFO, "task", message = %m);
        self.tasks.lock().unwrap().push(TimedSpan {
            span: sp,
            start: Instant::now(),
            label: m.to_string(),
        });
    }

    fn outro(&self, m: &str) {
        if let Some(s) = self.inner.outro(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Info, &s),
                LogFormat::Text => {
                    self.steps.lock().unwrap().clear();

                    let task = self.tasks.lock().unwrap().pop();
                    if let Some(TimedSpan { span, start, .. }) = task {
                        drop(span);
                        let elapsed = start.elapsed();
                        let timing = format_duration(elapsed);

                        let msg = if elapsed.as_millis() > 0 {
                            format!("{s} (took {timing})")
                        } else {
                            s.to_string()
                        };

                        let _ = self.backend.render_outro(&msg);

                        if self.inner.is_verbose() {
                            info!("{msg}");
                        }
                    }
                }
            }
        }
    }

    fn done(&self) {
        if let Some(s) = self.inner.done() {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Info, &s),
                LogFormat::Text => {
                    self.steps.lock().unwrap().clear();

                    let task = self.tasks.lock().unwrap().pop();
                    if let Some(TimedSpan { span, start, .. }) = task {
                        drop(span);
                        let elapsed = start.elapsed();
                        let timing = format_duration(elapsed);

                        let msg = if elapsed.as_millis() > 0 {
                            format!("{s} (took {timing})")
                        } else {
                            s.to_string()
                        };

                        let _ = self.backend.render_outro(&msg);

                        if self.inner.is_verbose() {
                            info!("{msg}");
                        }
                    }
                }
            }
        }
    }

    fn step(&self, m: &str) {
        if let Some(s) = self.inner.step(m) {
            match self.format {
                LogFormat::Json => {
                    self.emit_json(LogLevel::Info, &s);
                }
                LogFormat::Text => {
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
                LogFormat::Json => self.emit_json(LogLevel::Info, &s),
                LogFormat::Text => {
                    let _ = self.backend.render_success(&s);
                }
            }
        }
    }

    fn warn(&self, m: &str) {
        if let Some(s) = self.inner.warn(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Warn, &s),
                LogFormat::Text => {
                    let _ = self.backend.render_warning(&s);
                    warn!("{s}");
                }
            }
        }
    }

    fn err(&self, m: &str) {
        let s = self.inner.err(m);

        match self.format {
            LogFormat::Json => self.emit_json(LogLevel::Error, &s),
            LogFormat::Text => {
                let _ = self.backend.render_error(&s);
                error!("{s}");
            }
        }
    }

    fn info(&self, m: &str) {
        if let Some(s) = self.inner.info(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Info, &s),
                LogFormat::Text => {
                    let _ = self.backend.render_info(&s);
                }
            }
        }
    }

    fn dim(&self, m: &str) {
        if let Some(s) = self.inner.dim(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Debug, &s),
                LogFormat::Text => {
                    let _ = self.backend.render_remark(&s);
                }
            }
        }
    }

    fn debug(&self, m: &str) {
        if let Some(s) = self.inner.debug(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Debug, &s),
                LogFormat::Text => {
                    debug!("{s}");
                }
            }
        }
    }

    fn trace(&self, m: &str) {
        if let Some(s) = self.inner.trace(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Trace, &s),
                LogFormat::Text => {
                    trace!("{s}");
                }
            }
        }
    }

    fn dump_tree(&self) {
        self.dump_task_tree();
    }
}

// -----------------------------------------------------------------------------
// Structured Fields
// -----------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        }
    }
}

// -----------------------------------------------------------------------------
// LogEvent: builder for structured fields, emits on Drop
// -----------------------------------------------------------------------------
pub struct LogEvent<'a, L: FormatLogger, B: RenderBackend> {
    pub(crate) printer: &'a Printer<L, B>,
    pub(crate) level: LogLevel,
    pub(crate) message: String,
    pub(crate) fields: Fields,
    pub(crate) emitted: bool,
}

impl<'a, L: FormatLogger, B: RenderBackend> LogEvent<'a, L, B> {
    /// Constructor used by Printer builder APIs
    pub fn new(printer: &'a Printer<L, B>, level: LogLevel, msg: &str) -> Self {
        Self {
            printer,
            level,
            message: msg.to_string(),
            fields: Fields::new(),
            emitted: false,
        }
    }

    /// Add a single structured field
    pub fn field(mut self, key: impl Into<String>, value: impl ToString) -> Self {
        self.fields.insert(key.into(), value.to_string());
        self
    }

    /// Add multiple structured fields
    pub fn fields<I, K, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: ToString,
    {
        for (k, v) in iter {
            self.fields.insert(k.into(), v.to_string());
        }
        self
    }

    /// Optional explicit emission (rarely needed)
    pub fn emit(mut self) {
        if !self.emitted {
            self.printer
                .emit_event(self.level, &self.message, self.fields.clone());
            self.emitted = true;
        }
    }
}

impl<'a, L: FormatLogger, B: RenderBackend> Drop for LogEvent<'a, L, B> {
    fn drop(&mut self) {
        if self.emitted {
            return;
        }

        // Take fields so we don't clone
        let fields = std::mem::take(&mut self.fields);

        self.printer.emit_event(self.level, &self.message, fields);
        self.emitted = true;
    }
}

// -----------------------------------------------------------------------------
// Printer: unified emit_event, JSON helpers, and builder-style APIs
// -----------------------------------------------------------------------------
impl<L: FormatLogger, B: RenderBackend> Printer<L, B> {
    // -------------------------------------------------------------------------
    // JSON emission (single unified implementation)
    // -------------------------------------------------------------------------
    fn emit_json_fields(&self, level: LogLevel, message: &str, fields: Option<&Fields>) {
        let mut obj = serde_json::json!({
            "level": level.as_str(),
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        if let Some(f) = fields
            && !f.is_empty()
        {
            obj["fields"] = serde_json::to_value(f).unwrap();
        }

        match level {
            LogLevel::Error => eprintln!("{obj}"),
            _ => println!("{obj}"),
        }
    }

    fn emit_json(&self, level: LogLevel, message: &str) {
        self.emit_json_fields(level, message, None);
    }

    // -------------------------------------------------------------------------
    // Public: structured JSON logging (used by Drop-based LogEvent)
    // -------------------------------------------------------------------------
    pub fn emit_event(&self, level: LogLevel, msg: &str, fields: Fields) {
        match self.format {
            LogFormat::Json => self.emit_json_fields(level, msg, Some(&fields)),
            LogFormat::Text => self.emit_text(level, msg),
        }
    }

    // -------------------------------------------------------------------------
    // Text-mode emission
    // -------------------------------------------------------------------------
    fn emit_text(&self, level: LogLevel, msg: &str) {
        match level {
            LogLevel::Info => {
                if let Some(s) = self.inner.info(msg) {
                    let _ = self.backend.render_info(&s);
                }
            }
            LogLevel::Warn => {
                if let Some(s) = self.inner.warn(msg) {
                    let _ = self.backend.render_warning(&s);
                }
            }
            LogLevel::Error => {
                let s = self.inner.err(msg);
                let _ = self.backend.render_error(&s);
            }
            LogLevel::Debug => {
                if matches!(self.verbosity, Verbosity::Verbose | Verbosity::Trace)
                    && let Some(s) = self.inner.debug(msg)
                {
                    let _ = self.backend.render_debug(&s);
                }
            }
            LogLevel::Trace => {
                if self.verbosity == Verbosity::Trace
                    && let Some(s) = self.inner.trace(msg)
                {
                    let _ = self.backend.render_trace(&s);
                }
            }
        }
    }

    // -------------------------------------------------------------------------
    // Convenience: legacy API for structured fields
    // -------------------------------------------------------------------------
    pub fn info_with_fields(&self, m: &str, fields: Fields) {
        match self.format {
            LogFormat::Json => self.emit_json_fields(LogLevel::Info, m, Some(&fields)),
            LogFormat::Text => {
                // In text mode, fields are ignored — consistent with Drop-based LogEvent
                let _ = self.inner.info(m).map(|s| self.backend.render_info(&s));
            }
        }
    }

    // -------------------------------------------------------------------------
    // Builder-style APIs (Drop-based structured logging)
    // -------------------------------------------------------------------------
    pub fn info<'a>(&'a self, msg: &str) -> LogEvent<'a, L, B> {
        LogEvent::new(self, LogLevel::Info, msg)
    }

    pub fn warn<'a>(&'a self, msg: &str) -> LogEvent<'a, L, B> {
        LogEvent::new(self, LogLevel::Warn, msg)
    }

    pub fn error<'a>(&'a self, msg: &str) -> LogEvent<'a, L, B> {
        LogEvent::new(self, LogLevel::Error, msg)
    }

    pub fn debug<'a>(&'a self, msg: &str) -> LogEvent<'a, L, B> {
        LogEvent::new(self, LogLevel::Debug, msg)
    }

    pub fn trace<'a>(&'a self, msg: &str) -> LogEvent<'a, L, B> {
        LogEvent::new(self, LogLevel::Trace, msg)
    }
}

// -----------------------------------------------------------------------------
// Printer: add dump task tree
// -----------------------------------------------------------------------------
impl<L: FormatLogger, B: RenderBackend> Printer<L, B> {
    fn dump_task_tree(&self) {
        if !self.inner.is_verbose() {
            return;
        }

        let tasks = self.tasks.lock().unwrap();
        if tasks.is_empty() {
            println!("(no active tasks)");
            return;
        }

        println!("Active tasks:");
        for (i, t) in tasks.iter().enumerate() {
            let elapsed = t.start.elapsed();
            let timing = format_duration(elapsed);
            println!("  {}. {} (started, +{})", i + 1, t.label, timing);
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[cfg(test)]
#[path = "behavior_tests.rs"]
mod behavior_tests;
