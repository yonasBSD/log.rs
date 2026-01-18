//! # Behavior Tests for the Logging System
//!
//! These tests validate *runtime behavior* of the logger, including:
//!   1. Verbosity-level behavior (debug/trace visibility)
//!   2. That something is actually printed in verbose/trace modes
//!   3. That `Printer` forwards messages correctly
//!   4. JSON/Text formatting behavior under different verbosity levels
//!   5. Structured fields with drop-based API
//!   6. Nested spans, task trees, and timing information

use super::common::{capture_stderr, capture_stdout};
use crate::config;
use crate::logging::*;

// ============================================================================
// TEST HELPERS
// ============================================================================

fn make_printer<L: FormatLogger + 'static>(
    inner: L,
    format: LogFormat,
    verbosity: Verbosity,
) -> Printer<L, SimpleBackend> {
    Printer::new(inner, SimpleBackend, format, verbosity)
}

// ============================================================================
// 1. VERBOSITY BEHAVIOR TESTS
// ============================================================================

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
    fn quiet_hides_most_messages() {
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
    }

    #[test]
    fn quiet_preserves_errors() {
        config::setquiet(true);
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Quiet);

        let err = capture_stderr(|| {
            printer.err("boom");
        });

        assert!(err.contains("boom"));
    }
}

// ============================================================================
// 2. OUTPUT VERIFICATION TESTS
// ============================================================================

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

// ============================================================================
// 3. PRINTER FORWARDING TESTS
// ============================================================================

mod printer_forwarding_tests {
    use super::*;

    #[test]
    fn printer_ok_forwards_simple_logger_output() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer.ok("hello");
        });

        assert!(out.contains("hello"));
    }

    #[test]
    fn printer_warn_forwards_modern_logger_output() {
        let printer = make_printer(ModernLogger, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer.warn("careful");
        });

        assert!(out.contains("careful"));
    }

    #[test]
    fn printer_intro_creates_task_in_verbose() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("task");
        });

        assert!(out.contains("task"));
    }

    #[test]
    fn printer_step_creates_step_in_verbose() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.step("processing");
        });

        assert!(out.contains("processing"));
    }
}

// ============================================================================
// 4. JSON FORMAT TESTS
// ============================================================================

mod json_format_behavior_tests {
    use super::*;

    #[test]
    fn json_mode_produces_valid_json() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer.ok("hello");
        });

        for line in out.lines().filter(|l| !l.trim().is_empty()) {
            serde_json::from_str::<serde_json::Value>(line)
                .expect("Expected valid JSON output");
        }
    }

    #[test]
    fn json_mode_errors_are_valid_json() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Quiet);

        let out = capture_stderr(|| {
            printer.err("boom");
        });

        for line in out.lines().filter(|l| !l.trim().is_empty()) {
            serde_json::from_str::<serde_json::Value>(line)
                .expect("Expected valid JSON output");
        }
    }

    #[test]
    fn json_mode_outputs_json_for_spans() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Trace);

        let out = capture_stdout(|| {
            printer.intro("task");
            printer.step("step");
            printer.outro("done");
        });

        for line in out.lines().filter(|l| !l.trim().is_empty()) {
            serde_json::from_str::<serde_json::Value>(line)
                .expect("Expected valid JSON output");
        }
    }

    #[test]
    fn json_mode_includes_structured_fields() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Normal);

        let out = capture_stdout(|| {
            let mut fields = Fields::new();
            fields.insert("user_id".to_string(), "42".to_string());
            fields.insert("role".to_string(), "admin".to_string());
            printer.info_with_fields("User logged in", &fields);
        });

        let line = out
            .lines()
            .find(|l| !l.trim().is_empty())
            .expect("Expected output");
        let v: serde_json::Value = serde_json::from_str(line).expect("Expected valid JSON");

        assert_eq!(v["message"], "User logged in");
        assert_eq!(v["fields"]["user_id"], "42");
        assert_eq!(v["fields"]["role"], "admin");
    }
}

// ============================================================================
// 5. STRUCTURED FIELDS (DROP-BASED API)
// ============================================================================

mod structured_fields_tests {
    use super::*;

    #[test]
    fn json_mode_structured_fields_via_drop() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer
                .info("User logged in")
                .field("user_id", 7)
                .field("role", "admin");
        });

        let line = out
            .lines()
            .find(|l| !l.trim().is_empty())
            .expect("Expected output");
        let v: serde_json::Value = serde_json::from_str(line).expect("Expected valid JSON");

        assert_eq!(v["message"], "User logged in");
        assert_eq!(v["fields"]["user_id"], "7");
        assert_eq!(v["fields"]["role"], "admin");
    }

    #[test]
    fn text_mode_structured_fields_via_drop() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer
                .info("User logged in")
                .field("user_id", 7)
                .field("role", "admin");
        });

        assert!(out.contains("User logged in"));
        assert!(out.contains("user_id=7"));
        assert!(out.contains("role=admin"));
    }
}

// ============================================================================
// 6. NESTED SPANS AND TASK TREE TESTS
// ============================================================================

mod nested_span_tests {
    use super::*;

    #[test]
    fn nested_steps_clear_on_outro() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("top-level");
            printer.step("first-step");
            printer.step("second-step");
            printer.outro("done");
        });

        assert!(out.contains("top-level"));
        assert!(out.contains("first-step"));
        assert!(out.contains("second-step"));
        assert!(out.contains("done"));

        assert!(printer.steps.lock().unwrap().is_empty());
        assert!(printer.tasks.lock().unwrap().is_empty());
    }

    #[test]
    fn nested_tasks_clear_on_outro() {
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

        assert!(printer.tasks.lock().unwrap().is_empty());
    }

    #[test]
    fn dump_tree_shows_active_tasks() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("build");
            printer.intro("test");
            printer.dump_tree();
        });

        assert!(out.contains("Active tasks"));
        assert!(out.contains("build"));
        assert!(out.contains("test"));
    }
}

// ============================================================================
// 7. TIMING TESTS
// ============================================================================

mod timing_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn outro_includes_timing_in_verbose_mode() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("timed-task");
            std::thread::sleep(Duration::from_millis(20));
            printer.outro("finished");
        });

        assert!(out.contains("timed-task"));
        assert!(out.contains("finished"));
        assert!(
            out.contains("took"),
            "Expected timing information like '(took 20ms)' but got: {out}"
        );
    }

    #[test]
    fn nested_timing_tracks_independently() {
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
        assert!(out.contains("took"));
    }

    #[test]
    fn quiet_mode_preserves_timing_summaries() {
        config::setquiet(true);
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Quiet);

        let out = capture_stdout(|| {
            printer.intro("quiet-task");
            std::thread::sleep(Duration::from_millis(20));
            printer.outro("quiet-outro");

            printer.intro("another-task");
            std::thread::sleep(Duration::from_millis(20));
            printer.done();
        });

        // In quiet mode, intro is suppressed but outro/done timing is preserved
        assert!(out.contains("quiet-outro"));
        assert!(out.contains("Done!"));
        assert!(out.contains("took"));
    }
}
