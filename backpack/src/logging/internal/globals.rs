use crate::logging::*;
use once_cell::sync::OnceCell;

pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROJECT_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

/// A global, thread-safe screen logger.
pub trait GlobalLoggerType: EmitsEvents + Send + Sync {}

/// The erased global logger type used throughout the crate.
pub type GlobalLogger = dyn GlobalLoggerType;

static LOGGER: OnceCell<Box<dyn GlobalLoggerType>> = OnceCell::new();

/// One-time guard for tracing subscriber initialization.
pub static INIT: OnceCell<()> = OnceCell::new();

/// `LogProxy`
pub static L: LogProxy = LogProxy;

/// Set the global logger.
pub fn set_logger<L>(logger: L)
where
    L: GlobalLoggerType + 'static,
{
    let _ = LOGGER.set(Box::new(logger));
}

/// Retrieve the global logger.
pub fn logger() -> &'static dyn GlobalLoggerType {
    LOGGER
        .get()
        .map(|b| &**b)
        .expect("Logger not initialized")
}
