use crate::logging::{globals, logger, GlobalLogger, LogEvent, LogLevel};
use globals::{INIT, PROJECT_DESC, PROJECT_NAME};
use terminal_banner::Banner;
use tracing_subscriber::{
    filter::LevelFilter, fmt::writer::BoxMakeWriter, prelude::*, Layer, Registry,
};

/// Initialize the global tracing subscriber.
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    if INIT.get().is_some() {
        return Ok(());
    }

    INIT.set(()).ok();
    env_rs::init()?;

    let telemetry_fmt = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .without_time()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_writer(BoxMakeWriter::new(std::io::stderr));

    let registry = Registry::default().with(telemetry_fmt.with_filter(LevelFilter::TRACE));

    #[cfg(feature = "tokio-console")]
    let registry = registry.with(console_subscriber::spawn());

    tracing::subscriber::set_global_default(registry)?;

    if std::env::var("RUST_LOG").is_ok()
        && ["debug", "trace"].contains(&std::env::var("RUST_LOG").unwrap().to_lowercase().as_str())
    {
        let banner = Banner::new()
            .text(format!("Welcome to {PROJECT_NAME}!\n").into())
            .text(PROJECT_DESC.into())
            .render();

        println!("{banner}");
    }

    Ok(())
}

pub fn ok(msg: &str) -> LogEvent<'static, GlobalLogger> {
    LogEvent::new(logger(), LogLevel::Info, msg)
}

pub fn warn(msg: &str) -> LogEvent<'static, GlobalLogger> {
    LogEvent::new(logger(), LogLevel::Warn, msg)
}

pub fn err(msg: &str) -> LogEvent<'static, GlobalLogger> {
    LogEvent::new(logger(), LogLevel::Error, msg)
}

pub fn info(msg: &str) -> LogEvent<'static, GlobalLogger> {
    LogEvent::new(logger(), LogLevel::Info, msg)
}

pub fn dim(msg: &str) -> LogEvent<'static, GlobalLogger> {
    LogEvent::new(logger(), LogLevel::Info, msg)
}

pub fn intro(msg: &str) -> LogEvent<'static, GlobalLogger> {
    LogEvent::new(logger(), LogLevel::Info, msg)
}

pub fn outro(msg: &str) -> LogEvent<'static, GlobalLogger> {
    LogEvent::new(logger(), LogLevel::Info, msg)
}

pub fn done() -> LogEvent<'static, GlobalLogger> {
    LogEvent::new(logger(), LogLevel::Info, "done")
}

pub fn step(msg: &str) -> LogEvent<'static, GlobalLogger> {
    LogEvent::new(logger(), LogLevel::Info, msg)
}

pub fn debug(msg: &str) -> LogEvent<'static, GlobalLogger> {
    LogEvent::new(logger(), LogLevel::Debug, msg)
}

pub fn trace(msg: &str) -> LogEvent<'static, GlobalLogger> {
    LogEvent::new(logger(), LogLevel::Trace, msg)
}
