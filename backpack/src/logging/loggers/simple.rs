use crate::{config, logging::FormatLogger};

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
