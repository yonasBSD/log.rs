use crate::logging::tests::common::*;
use crate::logging::*;
use pretty_assertions::assert_eq;

#[test]
fn printer_initial_state_is_empty() {
    let logger = MockLogger::new(Verbosity::Normal);
    let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

    assert_eq!(printer.tasks.lock().unwrap().len(), 0);
    assert_eq!(printer.steps.lock().unwrap().len(), 0);
}

#[test]
fn printer_respects_json_format() {
    let logger = MockLogger::new(Verbosity::Normal);
    let printer = Printer::new(logger, SimpleBackend, LogFormat::Json, Verbosity::Normal);

    assert_eq!(printer.format, LogFormat::Json);
}

#[test]
fn printer_respects_text_format() {
    let logger = MockLogger::new(Verbosity::Normal);
    let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

    assert_eq!(printer.format, LogFormat::Text);
}

#[test]
fn printer_task_stack_initially_empty() {
    let logger = MockLogger::new(Verbosity::Verbose);
    let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

    assert!(printer.tasks.lock().unwrap().is_empty());
}

#[test]
fn printer_step_stack_initially_empty() {
    let logger = MockLogger::new(Verbosity::Verbose);
    let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

    assert!(printer.steps.lock().unwrap().is_empty());
}
