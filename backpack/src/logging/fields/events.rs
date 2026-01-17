// -----------------------------------------------------------------------------
// LogEvent: builder for structured fields, emits on Drop
// -----------------------------------------------------------------------------

use crate::logging::{FormatLogger, LogLevel, Printer, RenderBackend};
use std::collections::BTreeMap;

/// Structured fields attached to a log event.
pub type Fields = BTreeMap<String, String>;

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
                .emit_event(self.level, &self.message, &self.fields.clone());
            self.emitted = true;
        }
    }
}

impl<L: FormatLogger, B: RenderBackend> Drop for LogEvent<'_, L, B> {
    fn drop(&mut self) {
        if self.emitted {
            return;
        }

        // Take fields so we don't clone
        let fields = std::mem::take(&mut self.fields);

        self.printer.emit_event(self.level, &self.message, &fields);
        self.emitted = true;
    }
}
