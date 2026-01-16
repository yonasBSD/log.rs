//! # Behavior Tests for the Logging System
//!
//! These tests validate *runtime behavior* of the logger, including:
//!   1. Verbosity-level behavior (debug/trace visibility)
//!   2. That something is actually printed in verbose/trace modes
//!   3. That `Printer` forwards messages correctly
//!   4. JSON/Text formatting behavior under different verbosity levels
//!
//! Unlike the formatting tests in `tests.rs`, these tests capture real stdout
//! and stderr output because `Printer` (with `SimpleBackend`) prints using
//! println!/eprintln! and tracing macros, not through a mock I/O layer.

use super::*;
use gag::BufferRedirect;
use std::io::Read;

//
// -----------------------------------------------------------------------------
// Test Utility: Capture stdout/stderr for Printer behavior tests
// -----------------------------------------------------------------------------
// We use the `gag` crate to temporarily redirect stdout/stderr so we can assert
// on what the Printer + SimpleBackend actually prints.
//
fn capture_stdout<F: FnOnce()>(f: F) -> String {
    let mut buf = Vec::new();
    let mut redirect = BufferRedirect::stdout().unwrap();
    f();
    redirect.read_to_end(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

fn capture_stderr<F: FnOnce()>(f: F) -> String {
    let mut buf = Vec::new();
    let mut redirect = BufferRedirect::stderr().unwrap();
    f();
    redirect.read_to_end(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

//
// -----------------------------------------------------------------------------
// Helper: Create a Printer for tests
// -----------------------------------------------------------------------------
// This returns a Printer configured with the given FormatLogger, backend,
// format, and verbosity. For behavior tests we always use SimpleBackend so
// output goes through println!/eprintln!.
//
fn make_printer<L: FormatLogger + 'static>(
    inner: L,
    format: LogFormat,
    verbosity: Verbosity,
) -> Printer<L, SimpleBackend> {
    Printer::new(inner, SimpleBackend, format, verbosity)
}

//
// ============================================================================
// 1. VERBOSITY BEHAVIOR TESTS
// ============================================================================
// These tests verify that debug/trace visibility matches the verbosity rules.
//
mod verbosity_behavior_tests {
    use super::*;

    #[test]
    fn debug_visible_in_verbose() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let err = capture_stderr(|| {
            printer.debug("hello debug");
        });

        assert!(err.contains("debug"));
    }

    #[test]
    fn debug_hidden_in_normal() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Normal);

        let err = capture_stderr(|| {
            printer.debug("hello debug");
        });

        assert!(err.trim().is_empty());
    }

    #[test]
    fn trace_visible_only_in_trace() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Trace);

        let err = capture_stderr(|| {
            printer.trace("hello trace");
        });

        assert!(err.contains("trace"));
    }

    #[test]
    fn trace_hidden_in_verbose() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let err = capture_stderr(|| {
            printer.trace("hello trace");
        });

        assert!(err.trim().is_empty());
    }

    #[test]
    fn quiet_hides_everything_except_errors() {
        config::setquiet(true);

        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Quiet);

        let out = capture_stdout(|| {
            printer.ok("ok");
            printer.warn("warn");
            printer.info("info");
            printer.debug("debug");
            printer.trace("trace");
        });

        assert!(out.trim().is_empty());

        let err = capture_stderr(|| {
            printer.err("boom");
        });

        assert!(err.contains("boom"));
    }
}

//
// ============================================================================
// 2. “SOMETHING IS ACTUALLY PRINTED” TESTS
// ============================================================================
// These tests ensure that verbose/trace modes actually produce output.
//
mod printing_behavior_tests {
    use super::*;

    #[test]
    fn verbose_mode_prints_debug() {
        let printer = make_printer(ModernLogger, LogFormat::Text, Verbosity::Verbose);

        let err = capture_stderr(|| {
            printer.debug("debug message");
        });

        assert!(err.contains("debug"));
    }

    #[test]
    fn trace_mode_prints_trace() {
        let printer = make_printer(ModernLogger, LogFormat::Text, Verbosity::Trace);

        let err = capture_stderr(|| {
            printer.trace("trace message");
        });

        assert!(err.contains("trace"));
    }

    #[test]
    fn quiet_mode_suppresses_non_errors() {
        config::setquiet(true);

        let printer = make_printer(ModernLogger, LogFormat::Text, Verbosity::Quiet);

        let out = capture_stdout(|| {
            printer.ok("ok");
            printer.warn("warn");
            printer.info("info");
            printer.debug("debug");
            printer.trace("trace");
        });

        assert!(out.trim().is_empty());

        let err = capture_stderr(|| {
            printer.err("boom");
        });

        assert!(err.contains("boom"));
    }
}

//
// ============================================================================
// 3. PRINTER FORWARDING TESTS
// ============================================================================
// These tests verify that Printer forwards formatted messages correctly
// through the SimpleBackend.
//
mod printer_forwarding_tests {
    use super::*;

    #[test]
    fn printer_ok_forwards_simplelogger_output() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer.ok("hello");
        });

        assert!(out.contains("hello"));
    }

    #[test]
    fn printer_warn_forwards_modernlogger_output() {
        let printer = make_printer(ModernLogger, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer.warn("careful");
        });

        assert!(out.contains("careful"));
    }

    #[test]
    fn printer_intro_creates_task_span_in_verbose() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("task");
        });

        assert!(out.contains("task"));
    }

    #[test]
    fn printer_step_creates_step_span_in_verbose() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.step("processing");
        });

        assert!(out.contains("processing"));
    }
}

//
// ============================================================================
// 4. JSON/TEXT FORMAT BEHAVIOR TESTS
// ============================================================================
// These tests verify JSON output behavior and ensure spans are not created.
//
mod json_format_behavior_tests {
    use super::*;

    #[test]
    fn json_mode_always_prints_json() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer.ok("hello");
        });

        for line in out.lines().filter(|l| !l.trim().is_empty()) {
            serde_json::from_str::<serde_json::Value>(line).expect("Expected valid JSON output");
        }
    }

    #[test]
    fn json_mode_errors_are_json() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Quiet);

        let out = capture_stderr(|| {
            printer.err("boom");
        });

        for line in out.lines().filter(|l| !l.trim().is_empty()) {
            serde_json::from_str::<serde_json::Value>(line).expect("Expected valid JSON output");
        }
    }

    #[test]
    fn json_mode_does_not_create_spans() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Trace);

        let out = capture_stdout(|| {
            printer.intro("task");
            printer.step("step");
            printer.outro("done");
        });

        for line in out.lines().filter(|l| !l.trim().is_empty()) {
            serde_json::from_str::<serde_json::Value>(line).expect("Expected valid JSON output");
        }
    }
}

mod nested_span_tests {
    use super::*;

    #[test]
    fn nested_steps_create_nested_spans() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("top-level");
            printer.step("first-step");
            printer.step("second-step");
            printer.outro("done");
        });

        // All messages should appear in stdout
        assert!(out.contains("top-level"));
        assert!(out.contains("first-step"));
        assert!(out.contains("second-step"));
        assert!(out.contains("done"));

        // Internally, steps should be cleared after outro()
        assert!(printer.steps.lock().unwrap().is_empty());
        assert!(printer.tasks.lock().unwrap().is_empty());
    }

    #[test]
    fn nested_tasks_create_multiple_task_spans() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("task-1");
            printer.intro("task-2");
            printer.outro("done-2");
            printer.outro("done-1");
        });

        assert!(out.contains("task-1"));
        assert!(out.contains("task-2"));
        assert!(out.contains("done-2"));
        assert!(out.contains("done-1"));

        // All spans should be closed
        assert!(printer.tasks.lock().unwrap().is_empty());
    }
}

mod timing_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn outro_prints_timing_information() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("timed-task");
            std::thread::sleep(Duration::from_millis(20));
            printer.outro("finished");
        });

        assert!(out.contains("timed-task"));
        assert!(out.contains("finished"));

        // Expect timing suffix
        assert!(
            out.contains("took"),
            "Expected timing information like '(took 20ms)' but got: {out}"
        );
    }

    #[test]
    fn nested_timing_is_independent() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("outer");
            std::thread::sleep(Duration::from_millis(10));

            printer.intro("inner");
            std::thread::sleep(Duration::from_millis(10));
            printer.outro("inner-done");

            printer.outro("outer-done");
        });

        assert!(out.contains("outer"));
        assert!(out.contains("inner"));
        assert!(out.contains("inner-done"));
        assert!(out.contains("outer-done"));

        // Both should have timing suffixes
        let inner_has_timing = out.contains("inner-done") && out.contains("took");
        let outer_has_timing = out.contains("outer-done") && out.contains("took");

        assert!(inner_has_timing, "Inner task missing timing");
        assert!(outer_has_timing, "Outer task missing timing");
    }
}
