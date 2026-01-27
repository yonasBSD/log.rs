use crate::logging::*;
use insta::assert_snapshot;

#[test]
fn modern_logger_markers_snapshot() {
    let logger = ModernLogger;

    let out = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        logger.ok_raw("ok"),
        logger.warn_raw("warn"),
        logger.err_raw("err"),
        logger.info_raw("info"),
        logger.dim_raw("dim"),
    );

    assert_snapshot!(out);
}

#[test]
fn modern_logger_task_markers_snapshot() {
    let logger = ModernLogger;

    let out = format!(
        "{}\n{}\n{}\n{}\n",
        logger.intro_raw("intro"),
        logger.outro_raw("outro"),
        logger.done_raw(),
        logger.step_raw("step"),
    );

    assert_snapshot!(out);
}

#[test]
fn modern_logger_debug_and_trace_markers_snapshot() {
    let logger = ModernLogger;

    let out = format!(
        "{}\n{}\n",
        logger.debug_raw("debug"),
        logger.trace_raw("trace"),
    );

    assert_snapshot!(out);
}
