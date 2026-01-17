//! # Behavior Tests for the Logging System
//!
//! These tests validate *runtime behavior* of the logger, including:
//!   1. Verbosity-level behavior (debug/trace visibility)
//!   2. That something is actually printed in verbose/trace modes
//!   3. That `Printer` forwards messages correctly
//!   4. JSON/Text formatting behavior under different verbosity levels
//!   5. New behaviors: quiet-but-timed outro/done, structured fields, progress, task tree

use super::*;
use gag::BufferRedirect;
use std::io::Read;

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
    fn quiet_hides_everything_except_errors_and_timing_summaries() {
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

// ============================================================================
// 2. “SOMETHING IS ACTUALLY PRINTED” TESTS
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

        println!("==========> {err}");
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

// ============================================================================
// 4. JSON/TEXT FORMAT BEHAVIOR TESTS
// ============================================================================
mod json_format_behavior_tests {
    use super::*;

    #[test]
    fn json_mode_always_prints_valid_json() {
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
    fn json_mode_does_not_create_spans_but_still_outputs_json() {
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

    #[test]
    fn json_mode_supports_structured_fields() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Normal);

        let out = capture_stdout(|| {
            let mut fields = Fields::new();
            fields.insert("user_id".to_string(), "42".to_string());
            fields.insert("role".to_string(), "admin".to_string());
            printer.info_with_fields("User logged in", fields);
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
// STRUCTURED FIELDS (via drop)
// ============================================================================
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

// ============================================================================
// 5. NESTED SPAN / TASK TREE / TIMING TESTS
// ============================================================================
mod nested_span_tests {
    use super::*;

    #[test]
    fn nested_steps_create_nested_spans_and_clear_on_outro() {
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
    fn nested_tasks_create_multiple_task_spans_and_clear() {
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
    fn dump_tree_outputs_active_tasks_in_verbose_mode() {
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

mod timing_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn outro_prints_timing_information_in_verbose_mode() {
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
    fn nested_timing_is_independent_for_inner_and_outer_tasks() {
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
    fn quiet_mode_still_prints_timing_summaries_for_outro_and_done() {
        config::setquiet(true);
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Quiet);

        let out = capture_stdout(|| {
            printer.intro("quiet-task");
            std::thread::sleep(Duration::from_millis(20));
            printer.outro("quiet-outro");

            printer.intro("quiet-task");
            std::thread::sleep(Duration::from_millis(20));
            printer.done();
        });

        println!("{out}");

        // In quiet mode, intro is suppressed but outro timing summary is still printed.
        assert!(out.contains("quiet-outro"));
        assert!(out.contains("Done!"));
        assert!(out.contains("took"));
    }
}

// ============================================================================
// 6. PROGRESS API BEHAVIOR TESTS
// ============================================================================
mod progress_behavior_tests {
    use super::*;
    use std::sync::Once;

    static INIT_LOGGER: Once = Once::new();

    fn ensure_global_logger() {
        INIT_LOGGER.call_once(|| {
            let printer = Printer::new(
                SimpleLogger,
                SimpleBackend,
                LogFormat::Text,
                Verbosity::Normal,
            );
            crate::logging::set_logger(printer);
        });
    }

    #[test]
    fn progress_emits_intro_step_and_done_via_global_logger() {
        ensure_global_logger();

        let out = capture_stdout(|| {
            let mut p = crate::logging::L.progress("Downloading");
            p.update(1, 10);
            p.tick();
            p.finish("Done");
        });

        assert!(out.contains("Downloading"));
        assert!(out.contains("1/10"));
        assert!(out.contains("2/10"));
        assert!(out.contains("Done"));
    }
}

// ============================================================================
// 7. DEV-MODE BANNER (ROADMAP-LIKE, BUT IMPLEMENTED)
// ============================================================================
mod dev_mode_banner_tests {
    #[test]
    #[ignore]
    fn dev_mode_banner_prints_when_rust_log_is_debug_or_trace() {
        // This is tricky to test reliably because `init()` is global and only runs once.
        // Placeholder: when run in isolation with RUST_LOG=debug or trace, we expect
        // a banner containing the project name to be printed to stdout.
        //
        // You can turn this into a real test by:
        //   - spawning a subprocess with RUST_LOG=debug
        //   - capturing its stdout
        //   - asserting the banner is present
        assert!(true);
    }
}

// ============================================================================
// 8. ROADMAP FEATURE PLACEHOLDERS (IGNORED)
// ============================================================================
mod roadmap_behavior_tests {
    #[test]
    #[ignore]
    fn plugin_system_runtime_behavior_not_yet_implemented() {
        assert!(true);
    }

    #[test]
    #[ignore]
    fn compile_time_stripping_runtime_behavior_not_yet_implemented() {
        assert!(true);
    }

    #[test]
    #[ignore]
    fn log_capture_runtime_behavior_not_yet_implemented() {
        assert!(true);
    }

    #[test]
    #[ignore]
    fn opentelemetry_runtime_behavior_not_yet_implemented() {
        assert!(true);
    }

    #[test]
    #[ignore]
    fn sampling_runtime_behavior_not_yet_implemented() {
        assert!(true);
    }
}
