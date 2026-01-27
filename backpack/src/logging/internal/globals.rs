use crate::logging::{EmitsEvents, LogProxy, ScreenLogger};
use std::sync::OnceLock;

pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROJECT_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

/// A global, thread-safe screen logger.
pub trait GlobalLoggerType: EmitsEvents + ScreenLogger + Send + Sync + std::any::Any {}

/// The erased global logger type used throughout the crate.
pub type GlobalLogger = dyn GlobalLoggerType;

static mut LOGGER: Option<&'static dyn GlobalLoggerType> = None;
pub static INIT: OnceLock<()> = OnceLock::new();

/// `LogProxy`
pub static L: LogProxy = LogProxy;

pub fn set_logger<L>(logger: L)
where
    L: GlobalLoggerType + 'static,
{
    let boxed = Box::new(logger);
    let leaked: &'static dyn GlobalLoggerType = Box::leak(boxed);

    unsafe {
        LOGGER = Some(leaked);
    }
}

pub fn logger() -> &'static dyn GlobalLoggerType {
    unsafe { LOGGER.expect("Logger not initialized") }
}

#[cfg(test)]
pub fn reset_logger() {
    unsafe {
        LOGGER = None;
    }
}
