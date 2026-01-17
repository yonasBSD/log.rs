use crate::logging::{globals, log};
use globals::{INIT, PROJECT_DESC, PROJECT_NAME};
use terminal_banner::Banner;
use tracing_subscriber::{
    Layer, Registry, filter::LevelFilter, fmt::writer::BoxMakeWriter, prelude::*,
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
        .with_thread_ids(true)
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

pub fn ok(msg: &str) {
    log().ok(msg);
}

pub fn warn(msg: &str) {
    log().warn(msg);
}

pub fn err(msg: &str) {
    log().err(msg);
}

pub fn info(msg: &str) {
    log().info(msg);
}

pub fn dim(msg: &str) {
    log().dim(msg);
}

pub fn intro(msg: &str) {
    log().intro(msg);
}

pub fn outro(msg: &str) {
    log().outro(msg);
}

pub fn done() {
    log().done();
}

pub fn step(msg: &str) {
    log().step(msg);
}

pub fn debug(msg: &str) {
    log().debug(msg);
}

pub fn trace(msg: &str) {
    log().trace(msg);
}
