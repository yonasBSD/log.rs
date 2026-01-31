use crate::logging::{
    EmitsEvents, Fields, FormatLogger, LogEvent, LogLevel, Printer, RenderBackend, TimestampMode,
};
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
        });

        let timestamp = *self.timestamp.lock().unwrap();
        match timestamp {
            TimestampMode::Real => {
                obj["timestamp"] = serde_json::Value::String(chrono::Utc::now().to_rfc3339());
            }
            TimestampMode::Disabled => {
                // do nothing
            }
            TimestampMode::Fixed(value) => {
                obj["timestamp"] = serde_json::Value::String(value.to_string());
            }
        }

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
    // Text-mode emission with fields support
    // -------------------------------------------------------------------------
    pub fn emit_text_fields(&self, level: LogLevel, msg: &str, fields: Option<&Fields>) {
        // Format the message with fields appended if present
        let formatted_msg = if let Some(f) = fields
            && !f.is_empty()
        {
            let fields_str = f
                .iter()
                .map(|(k, v)| format!("\x1b[2m{k}={v}\x1b[0m")) // dim style
                .collect::<Vec<_>>()
                .join(" ");
            format!("{msg} {fields_str}")
        } else {
            msg.to_string()
        };

        match level {
            LogLevel::Info => {
                if let Some(s) = self.inner.info(&formatted_msg) {
                    let _ = self.backend.render_info(&s);
                }
            }
            LogLevel::Warn => {
                if let Some(s) = self.inner.warn(&formatted_msg) {
                    let _ = self.backend.render_warning(&s);
                }
            }
            LogLevel::Error => {
                let s = self.inner.err(&formatted_msg);
                let _ = self.backend.render_error(&s);
            }
            LogLevel::Debug => {
                if matches!(self.verbosity, Verbosity::Verbose | Verbosity::Trace)
                    && let Some(s) = self.inner.debug(&formatted_msg)
                {
                    let _ = self.backend.render_debug(&s);
                }
            }
            LogLevel::Trace => {
                if self.verbosity == Verbosity::Trace
                    && let Some(s) = self.inner.trace(&formatted_msg)
                {
                    let _ = self.backend.render_trace(&s);
                }
            }
            LogLevel::Progress => {
                println!("{formatted_msg}");
            }
        }
    }

    pub fn emit_text(&self, level: LogLevel, msg: &str) {
        self.emit_text_fields(level, msg, None);
    }

    // -------------------------------------------------------------------------
    // Public: structured logging (used by Drop-based LogEvent)
    // -------------------------------------------------------------------------
    pub fn emit_event(&self, level: LogLevel, msg: &str, fields: &Fields) {
        match self.format {
            LogFormat::Json => self.emit_json_fields(level, msg, Some(fields)),
            LogFormat::Text => self.emit_text_fields(level, msg, Some(fields)),
        }
    }

    // -------------------------------------------------------------------------
    // Convenience: legacy API for structured fields
    // -------------------------------------------------------------------------
    pub fn info_with_fields(&self, m: &str, fields: &Fields) {
        match self.format {
            LogFormat::Json => self.emit_json_fields(LogLevel::Info, m, Some(fields)),
            LogFormat::Text => self.emit_text_fields(LogLevel::Info, m, Some(fields)),
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

    pub fn done_event(&self) -> LogEvent<'_, Self> {
        LogEvent::new(self, LogLevel::Trace, "")
    }

    pub fn dump_tree_event(&self) -> LogEvent<'_, Self> {
        LogEvent::new(self, LogLevel::Debug, "")
    }
}

// Let Printer be a source of structured events for LogEvent
impl<L: FormatLogger, B: RenderBackend> EmitsEvents for Printer<L, B> {
    fn emit_event(&self, level: LogLevel, msg: &str, fields: &crate::logging::Fields) {
        match self.format {
            LogFormat::Json => self.emit_json_fields(level, msg, Some(fields)),
            LogFormat::Text => self.emit_text_fields(level, msg, Some(fields)),
        }
    }
}
