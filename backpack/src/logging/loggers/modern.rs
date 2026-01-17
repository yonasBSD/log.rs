use crate::logging::FormatLogger;

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
        "✔ Done!".to_string()
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
