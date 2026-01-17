use crate::logging::{Progress, log};

/// Proxy value so callers can write `L.ok("msg")` or `log().ok("msg")`.
pub struct LogProxy;

impl LogProxy {
    pub fn ok(&self, msg: &str) {
        log().ok(msg);
    }

    pub fn warn(&self, msg: &str) {
        log().warn(msg);
    }

    pub fn err(&self, msg: &str) {
        log().err(msg);
    }

    pub fn info(&self, msg: &str) {
        log().info(msg);
    }

    pub fn dim(&self, msg: &str) {
        log().dim(msg);
    }

    pub fn intro(&self, msg: &str) {
        log().intro(msg);
    }

    pub fn outro(&self, msg: &str) {
        log().outro(msg);
    }

    pub fn done(&self) {
        log().done();
    }

    pub fn step(&self, msg: &str) {
        log().step(msg);
    }

    pub fn debug(&self, msg: &str) {
        log().debug(msg);
    }

    pub fn trace(&self, msg: &str) {
        log().trace(msg);
    }

    /// Dump the current task tree (verbose/trace only).
    pub fn dump_tree(&self) {
        log().dump_tree();
    }

    /// Start a progress handle for a long-running task.
    #[must_use]
    pub fn progress(&self, msg: &str) -> Progress {
        Progress::new(msg)
    }
}
