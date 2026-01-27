use crate::logging::*;

/// Proxy value so callers can write `L.ok("msg")` or `logger().ok("msg")`.
pub struct LogProxy;

impl LogProxy {
    pub fn ok(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, msg)
    }

    pub fn warn(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Warn, msg)
    }

    pub fn err(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Error, msg)
    }

    pub fn info(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, msg)
    }

    pub fn dim(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, msg)
    }

    pub fn intro(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, msg)
    }

    pub fn outro(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, msg)
    }

    pub fn done(&self) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, "done")
    }

    pub fn step(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, msg)
    }

    pub fn debug(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Debug, msg)
    }

    pub fn trace(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Trace, msg)
    }

    /// Dump the current task tree (verbose/trace only).
    pub fn dump_tree(&self) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Debug, "dump_tree")
    }

    /// Start a progress handle for a long-running task.
    #[must_use]
    pub fn progress(&self, label: &str, current: u64, total: Option<u64>, finished: bool) {
        let logger = crate::logging::logger();
        logger.progress(label, current, total, finished);
    }
}
