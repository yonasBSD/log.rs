use crate::logging::{done, intro, step, outro};

/// Lightweight progress handle for long-running tasks.
pub struct Progress {
    pub(crate) label: String,
    pub(crate) total: Option<u64>,
    pub(crate) current: u64,
    pub(crate) finished: bool,
}

impl Progress {
    /// Create a progress handle without a known total.
    pub fn new(label: &str) -> Self {
        intro(label);
        Self {
            label: label.to_string(),
            total: None,
            current: 0,
            finished: false,
        }
    }

    /// Create a progress handle with a known total.
    pub fn with_total(label: &str, total: u64) -> Self {
        intro(label);
        Self {
            label: label.to_string(),
            total: Some(total),
            current: 0,
            finished: false,
        }
    }

    /// Manually update progress with an explicit current/total.
    pub fn update(&mut self, current: u64, total: u64) {
        self.current = current;
        self.total = Some(total);
        let msg = format!("{} ({}/{})", self.label, current, total);
        step(&msg);
    }

    /// Increment progress by 1 and emit a step message.
    pub fn tick(&mut self) {
        self.current += 1;
        if let Some(total) = self.total {
            let msg = format!("{} ({}/{})", self.label, self.current, total);
            step(&msg);
        } else {
            step(&self.label);
        }
    }

    /// Finish the progress with a final message and a done marker.
    pub fn finish(mut self, msg: &str) {
        if !self.finished {
            outro(msg);
            done();
            self.finished = true;
        }
    }
}
