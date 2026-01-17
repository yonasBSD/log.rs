use crate::logging::{LogProxy, ScreenLogger};
use once_cell::sync::OnceCell;
use std::sync::Arc;

pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROJECT_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

/// A global, thread-safe screen logger.
pub static LOGGER: OnceCell<Arc<dyn ScreenLogger + Send + Sync>> = OnceCell::new();

/// One-time guard for tracing subscriber initialization.
pub static INIT: OnceCell<()> = OnceCell::new();

/// `LogProxy`
pub static L: LogProxy = LogProxy;

/// Set the global logger.
pub fn set_logger<L: ScreenLogger + Send + Sync + 'static>(logger: L) {
    let _ = LOGGER.set(Arc::new(logger));
}

/// Retrieve the global logger.
pub fn log() -> &'static Arc<dyn ScreenLogger + Send + Sync> {
    LOGGER.get().expect("Logger not initialized")
}
