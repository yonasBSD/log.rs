use crate::logging::*;
use crate::{LogFormat, Verbosity};

// -----------------------------------------------------------------------------
// Printer: unified emit_event, JSON helpers, and builder-style APIs
// -----------------------------------------------------------------------------
impl<L: FormatLogger, B: RenderBackend> Printer<L, B> {
    // -------------------------------------------------------------------------
    // JSON emission (single unified implementation)
    // -------------------------------------------------------------------------
    pub fn emit_json_fields(&self, level: LogLevel, message: &str, fields: Option<&Fields>) {
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

    pub fn emit_json(&self, level: LogLevel, message: &str) {
        self.emit_json_fields(level, message, None);
    }

    // -------------------------------------------------------------------------
    // Public: structured JSON logging (used by Drop-based LogEvent)
    // -------------------------------------------------------------------------
    pub fn emit_event(&self, level: LogLevel, msg: &str, fields: &Fields) {
        match self.format {
            LogFormat::Json => self.emit_json_fields(level, msg, Some(fields)),
            LogFormat::Text => self.emit_text(level, msg),
        }
    }

    // -------------------------------------------------------------------------
    // Text-mode emission
    // -------------------------------------------------------------------------
    pub fn emit_text(&self, level: LogLevel, msg: &str) {
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
    pub fn info_with_fields(&self, m: &str, fields: &Fields) {
        match self.format {
            LogFormat::Json => self.emit_json_fields(LogLevel::Info, m, Some(fields)),
            LogFormat::Text => {
                // In text mode, fields are ignored — consistent with Drop-based LogEvent
                let _ = self.inner.info(m).map(|s| self.backend.render_info(&s));
            }
        }
    }

    // -------------------------------------------------------------------------
    // Builder-style APIs (Drop-based structured logging)
    // -------------------------------------------------------------------------
    pub fn info<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Info, msg)
    }

    pub fn warn<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Warn, msg)
    }

    pub fn error<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Error, msg)
    }

    pub fn debug<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Debug, msg)
    }

    pub fn trace<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Trace, msg)
    }

    pub fn ok_event<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Info, msg)
    }

    pub fn warn_event<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Warn, msg)
    }

    pub fn err_event<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Error, msg)
    }

    pub fn info_event<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Info, msg)
    }

    pub fn dim_event<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Debug, msg)
    }

    pub fn debug_event<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Debug, msg)
    }

    pub fn trace_event<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Trace, msg)
    }

    pub fn intro_event<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Trace, msg)
    }

    pub fn step_event<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Trace, msg)
    }

    pub fn outro_event<'a>(&'a self, msg: &str) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Trace, msg)
    }

    pub fn done_event<'a>(&'a self) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Trace, "")
    }

    pub fn dump_tree_event<'a>(&'a self) -> LogEvent<'a, Self> {
        LogEvent::new(self, LogLevel::Debug, "")
    }
}

// Let Printer be a source of structured events for LogEvent
impl<L: FormatLogger, B: RenderBackend> EmitsEvents for Printer<L, B> {
    fn emit_event(&self, level: LogLevel, msg: &str, fields: &crate::logging::Fields) {
        match self.format {
            LogFormat::Json => {
                // JSON mode: include fields
                self.emit_json_fields(level, msg, Some(fields));
            }
            LogFormat::Text => {
                // Text mode: ignore fields for now, just emit the message
                self.emit_text(level, msg);
            }
        }
    }
}
