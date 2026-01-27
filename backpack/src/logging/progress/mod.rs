use crate::logging::L;

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
        // Keep the intro semantics you already had
        crate::logging::intro(label);

        Self {
            label: label.to_string(),
            total: None,
            current: 0,
            finished: false,
        }
    }

    /// Create a progress handle with a known total.
    pub fn with_total(label: &str, total: u64) -> Self {
        crate::logging::intro(label);

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

        // Semantic progress event; backend decides how to render
        let _ = L.progress(&self.label, self.current, self.total, false);
    }

    /// Increment progress by 1 and emit an update.
    pub fn tick(&mut self) {
        self.current += 1;
        let _ = L.progress(&self.label, self.current, self.total, false);
    }

    /// Finish the progress with a final message.
    ///
    /// `msg` is the final label shown by the backend (e.g. "Done", "Completed").
    pub fn finish(mut self, msg: &str) {
        if self.finished {
            return;
        }

        // Final progress event, marked as finished
        let _ = L.progress(msg, self.current, self.total, true);

        // Preserve your existing outro/done semantics for non-progress-aware backends
        crate::logging::outro(msg);
        crate::logging::done();

        self.finished = true;
    }
}
