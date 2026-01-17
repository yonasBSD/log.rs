/*
/// A lightweight progress handle.
///
/// This is intentionally simple: it just emits step/info/done messages
/// through the global logger, so it works with any backend.

use crate::log;

pub struct Progress {
    label: String,
    current: u64,
    total: Option<u64>,
}

impl Progress {
    #[must_use]
    pub fn new(label: &str) -> Self {
        log::<GlobalLogger>().intro(label);
        Self {
            label: label.to_string(),
            current: 0,
            total: None,
        }
    }

    #[must_use]
    pub fn with_total(label: &str, total: u64) -> Self {
        log::<GlobalLogger>().intro(label);
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
        log::<GlobalLogger>().step(&msg);
    }

    pub fn tick(&mut self) {
        self.current += 1;
        if let Some(total) = self.total {
            let msg = format!("{}: {}/{}", self.label, self.current, total);
            log::<GlobalLogger>().step(&msg);
        } else {
            let msg = format!("{}: {}", self.label, self.current);
            log::<GlobalLogger>().step(&msg);
        }
    }

    pub fn finish(self, _msg: &str) {
        log::<GlobalLogger>().done();
    }
}
*/
