use crate::logging::{GlobalLogger, LogEvent, LogLevel, logger};

/// Proxy value so callers can write `L.ok("msg")` or `logger().ok("msg")`.
pub struct LogProxy;

impl LogProxy {
    #[must_use]
    pub fn ok(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, msg)
    }

    #[must_use]
    pub fn warn(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Warn, msg)
    }

    #[must_use]
    pub fn err(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Error, msg)
    }

    #[must_use]
    pub fn info(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, msg)
    }

    #[must_use]
    pub fn dim(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, msg)
    }

    #[must_use]
    pub fn intro(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, msg)
    }

    #[must_use]
    pub fn outro(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, msg)
    }

    #[must_use]
    pub fn done(&self) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, "done")
    }

    #[must_use]
    pub fn step(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Info, msg)
    }

    #[must_use]
    pub fn debug(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Debug, msg)
    }

    #[must_use]
    pub fn trace(&self, msg: &str) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Trace, msg)
    }

    /// Dump the current task tree (verbose/trace only).
    #[must_use]
    pub fn dump_tree(&self) -> LogEvent<'static, GlobalLogger> {
        LogEvent::new(logger(), LogLevel::Debug, "dump_tree")
    }

    /// Start a progress handle for a long-running task.
    pub fn progress(&self, label: &str, current: u64, total: Option<u64>, finished: bool) {
        let logger = crate::logging::logger();
        logger.progress(label, current, total, finished);
    }
}
