/// Cargo-style verbosity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,   // -q
    Normal,  // default
    Verbose, // -v
    Trace,   // -vv
}

/// Output format for the logger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Text,
    Json,
}
