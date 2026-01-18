use crate::logging::*;

/// Proxy value so callers can write `L.ok("msg")` or `logger().ok("msg")`.
pub struct LogProxy;

impl LogProxy {
    pub fn ok(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        logger().ok_event(msg)
    }

    pub fn warn(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        logger().warn_event(msg)
    }

    pub fn err(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        logger().err_event(msg)
    }

    pub fn info(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        logger().info_event(msg)
    }

    pub fn dim(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        logger().dim_event(msg)
    }

    pub fn intro(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        logger().intro_event(msg)
    }

    pub fn outro(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        logger().outro_event(msg)
    }

    pub fn done(&self) -> LogEvent<'static, GlobalLogger> {
        logger().done_event()
    }

    pub fn step(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        logger().step_event(msg)
    }

    pub fn debug(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        logger().debug_event(msg)
    }

    pub fn trace(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        logger().trace_event(msg)
    }

    /// Dump the current task tree (verbose/trace only).
    pub fn dump_tree(&self) -> LogEvent<'static, GlobalLogger> {
        logger().dump_tree_event()
    }

    /*
    /// Start a progress handle for a long-running task.
    #[must_use]
    pub fn progress(&self, msg: &str) -> Progress {
        Progress::new(msg)
    }
    */
}
