use crate::logging::*;
use once_cell::sync::OnceCell;

pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROJECT_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

/// A global, thread-safe screen logger.
pub trait GlobalLoggerType: EmitsEvents + Send + Sync {}

pub type GlobalLogger = Printer<ModernLogger, ModernBackend>;

static LOGGER: OnceCell<GlobalLogger> = OnceCell::new();

/// One-time guard for tracing subscriber initialization.
pub static INIT: OnceCell<()> = OnceCell::new();

/// `LogProxy`
pub static L: LogProxy = LogProxy;

/// Set the global logger.
pub fn set_logger(logger: GlobalLogger) {
    let _ = LOGGER.set(logger);
}

/// Retrieve the global logger.
pub fn logger() -> &'static GlobalLogger {
    LOGGER.get().expect("Logger not initialized")
}
