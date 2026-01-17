use crate::config;

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
