// -----------------------------------------------------------------------------
// LogEvent: builder for structured fields, emits on Drop
// -----------------------------------------------------------------------------

use crate::logging::LogLevel;
use std::collections::BTreeMap;

/// A logger that can emit structured events.
pub trait EmitsEvents {
    fn emit_event(&self, level: LogLevel, msg: &str, fields: &Fields);
}

impl<L: EmitsEvents> Drop for LogEvent<'_, L> {
    fn drop(&mut self) {
        if self.emitted {
            return;
        }

        let fields = std::mem::take(&mut self.fields);
        self.logger.emit_event(self.level, &self.message, &fields);
        self.emitted = true;
    }
}

/// Structured fields attached to a log event.
pub type Fields = BTreeMap<String, String>;

pub struct LogEvent<'a, L: EmitsEvents> {
    logger: &'a L,
    level: LogLevel,
    message: String,
    fields: Fields,
    emitted: bool,
}

impl<'a, L: EmitsEvents> LogEvent<'a, L> {
    pub fn new(logger: &'a L, level: LogLevel, msg: &str) -> Self
    where
        L: EmitsEvents,
    {
        Self {
            logger,
            level,
            message: msg.to_string(),
            fields: Fields::new(),
            emitted: false,
        }
    }

    pub fn field(mut self, key: impl Into<String>, value: impl ToString) -> Self {
        self.fields.insert(key.into(), value.to_string());
        self
    }

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

    pub fn emit(mut self) {
        if !self.emitted {
            self.logger
                .emit_event(self.level, &self.message, &self.fields);
            self.emitted = true;
        }
    }
}
