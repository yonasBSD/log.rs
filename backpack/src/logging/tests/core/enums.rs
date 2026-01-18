
use crate::logging::{*, tests::common::*};
use pretty_assertions::assert_eq;

#[test]
fn verbosity_equality_and_ordering() {
    assert_eq!(Verbosity::Quiet, Verbosity::Quiet);
    assert_ne!(Verbosity::Quiet, Verbosity::Normal);
    assert_ne!(Verbosity::Normal, Verbosity::Verbose);
    assert_ne!(Verbosity::Verbose, Verbosity::Trace);
}

#[test]
fn log_format_equality() {
    assert_eq!(LogFormat::Text, LogFormat::Text);
    assert_ne!(LogFormat::Text, LogFormat::Json);
}

#[test]
fn verbosity_hierarchy_flags_match() {
    let quiet = MockLogger::new(Verbosity::Quiet);
    let normal = MockLogger::new(Verbosity::Normal);
    let verbose = MockLogger::new(Verbosity::Verbose);
    let trace = MockLogger::new(Verbosity::Trace);

    assert!(quiet.is_quiet());
    assert!(!quiet.is_verbose());

    assert!(!normal.is_quiet());
    assert!(!normal.is_verbose());

    assert!(!verbose.is_quiet());
    assert!(verbose.is_verbose());

    assert!(!trace.is_quiet());
    assert!(trace.is_verbose());
}

