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

pub(crate) mod backends;
pub(crate) mod fields;
pub mod internal;
pub(crate) mod loggers;
pub(crate) mod printers;
pub(crate) mod progress;

pub use backends::*;
pub use fields::*;
pub use internal::*;
pub use loggers::*;
pub use printers::*;
pub use progress::*;

/*
#[cfg(test)]
#[path = "tests/general.rs"]
mod general_tests;

#[cfg(test)]
#[path = "tests/behavior.rs"]
mod behavior_tests;
*/
