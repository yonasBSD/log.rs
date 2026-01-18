use crate::logging::*;

pub fn make_printer<L: FormatLogger + 'static>(
    inner: L,
    format: LogFormat,
    verbosity: Verbosity,
) -> Printer<L, SimpleBackend> {
    Printer::new(inner, SimpleBackend, format, verbosity)
}
