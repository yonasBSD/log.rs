
use crate::logging::*;
use insta::assert_snapshot;

#[test]
fn simple_logger_basic_markers_snapshot() {
    let logger = SimpleLogger;

    let out = format!(
        "{}\n{}\n{}\n{}\n",
        logger.ok_raw("ok"),
        logger.warn_raw("warn"),
        logger.err_raw("err"),
        logger.info_raw("info"),
    );

    assert_snapshot!(out);
}

#[test]
fn simple_logger_intro_outro_snapshot() {
    let logger = SimpleLogger;

    let intro = logger.intro_raw("Starting task");
    let outro = logger.outro_raw("Task complete");

    let out = format!("intro={intro}\noutro={outro}\n");
    assert_snapshot!(out);
}

#[test]
fn simple_logger_step_contains_message() {
    let logger = SimpleLogger;
    let step = logger.step_raw("Processing item");
    assert!(step.contains("Processing item"));
}

