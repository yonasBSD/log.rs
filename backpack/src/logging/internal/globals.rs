use crate::logging::*;
use once_cell::sync::OnceCell;

pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROJECT_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

/// A global, thread-safe screen logger.
pub trait GlobalLoggerType: EmitsEvents {}
pub type GlobalLogger = Printer<ModernLogger, ModernBackend>;
static LOGGER: OnceCell<Box<dyn GlobalLoggerType>> = OnceCell::new();

/// One-time guard for tracing subscriber initialization.
pub static INIT: OnceCell<()> = OnceCell::new();

/// `LogProxy`
pub static L: LogProxy = LogProxy;

/// Set the global logger.
pub fn set_logger<L: GlobalLoggerType + 'static>(logger: L) {
    let _ = LOGGER.set(Box::new(logger));
}

/// Retrieve the global logger.
pub fn log<L: GlobalLoggerType>() -> &'static L {
    LOGGER
        .get()
        .expect("Logger not initialized")
        .downcast_ref::<L>()
        .expect("Global logger type mismatch")
}
