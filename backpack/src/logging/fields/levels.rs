#[derive(Copy, Clone, Debug)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
    Trace,
}

impl LogLevel {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }
}
