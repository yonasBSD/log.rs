use crate::logging::*;

/// Mock FormatLogger for exercising default methods without real I/O.
pub struct MockLogger {
    quiet: bool,
    verbose: bool,
}

impl MockLogger {
    pub fn new(verbosity: Verbosity) -> Self {
        Self {
            quiet: verbosity == Verbosity::Quiet,
            verbose: matches!(verbosity, Verbosity::Verbose | Verbosity::Trace),
        }
    }
}

impl FormatLogger for MockLogger {
    fn is_quiet(&self) -> bool {
        self.quiet
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }

    fn ok_raw(&self, m: &str) -> String {
        format!("OK: {}", m)
    }

    fn warn_raw(&self, m: &str) -> String {
        format!("WARN: {}", m)
    }

    fn err_raw(&self, m: &str) -> String {
        format!("ERR: {}", m)
    }

    fn info_raw(&self, m: &str) -> String {
        format!("INFO: {}", m)
    }

    fn dim_raw(&self, m: &str) -> String {
        format!("DIM: {}", m)
    }

    fn intro_raw(&self, m: &str) -> String {
        format!("INTRO: {}", m)
    }

    fn outro_raw(&self, m: &str) -> String {
        format!("OUTRO: {}", m)
    }

    fn done_raw(&self) -> String {
        "DONE!".to_string()
    }

    fn step_raw(&self, m: &str) -> String {
        format!("STEP: {}", m)
    }

    fn debug_raw(&self, m: &str) -> String {
        format!("DEBUG: {}", m)
    }

    fn trace_raw(&self, m: &str) -> String {
        format!("TRACE: {}", m)
    }
}
