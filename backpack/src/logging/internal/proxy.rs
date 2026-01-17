use crate::logging::*;

/// Proxy value so callers can write `L.ok("msg")` or `log::<GlobalLogger>().ok("msg")`.
pub struct LogProxy;

impl LogProxy {
    pub fn ok(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        log::<GlobalLogger>().ok_event(msg)
    }

    pub fn warn(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        log::<GlobalLogger>().warn_event(msg)
    }

    pub fn err(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        log::<GlobalLogger>().err_event(msg)
    }

    pub fn info(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        log::<GlobalLogger>().info_event(msg)
    }

    pub fn dim(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        log::<GlobalLogger>().dim_event(msg)
    }

    pub fn intro(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        log::<GlobalLogger>().intro_event(msg)
    }

    pub fn outro(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        log::<GlobalLogger>().outro_event(msg)
    }

    pub fn done(&self) -> LogEvent<'static, GlobalLogger> {
        log::<GlobalLogger>().done_event()
    }

    pub fn step(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        log::<GlobalLogger>().step_event(msg)
    }

    pub fn debug(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        log::<GlobalLogger>().debug_event(msg)
    }

    pub fn trace(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        log::<GlobalLogger>().trace_event(msg)
    }

    /// Dump the current task tree (verbose/trace only).
    pub fn dump_tree(&self) -> LogEvent<'static, GlobalLogger> {
        log::<GlobalLogger>().dump_tree_event()
    }

    /*
    /// Start a progress handle for a long-running task.
    #[must_use]
    pub fn progress(&self, msg: &str) -> Progress {
        Progress::new(msg)
    }
    */
}
