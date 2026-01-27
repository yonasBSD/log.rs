use crate::logging::*;
use insta::assert_snapshot;

#[test]
fn simple_logger_workflow_snapshot() {
    let logger = SimpleLogger;

    let intro = logger.intro_raw("Starting deployment");
    let step1 = logger.step_raw("Building assets");
    let step2 = logger.step_raw("Uploading files");
    let outro = logger.outro_raw("Deployment complete");

    let out = format!("{intro}\n{step1}\n{step2}\n{outro}\n");
    assert_snapshot!(out);
}

#[test]
fn modern_logger_workflow_snapshot() {
    let logger = ModernLogger;

    let intro = logger.intro_raw("Running tests");
    let step = logger.step_raw("Test suite 1");
    let ok = logger.ok_raw("All tests passed");
    let outro = logger.outro_raw("Testing complete");

    let out = format!("{intro}\n{step}\n{ok}\n{outro}\n");
    assert_snapshot!(out);
}

#[test]
fn error_always_visible_snapshot() {
    let logger = SimpleLogger;

    let err1 = logger.err_raw("Critical error");
    let err2 = logger.err_raw("Critical error");

    assert_eq!(err1, err2);
    assert!(err1.contains("Critical error"));

    let out = format!("{err1}\n{err2}\n");
    assert_snapshot!(out);
}
