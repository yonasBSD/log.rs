
//! Behavior tests for runtime logging behavior:
//!   1. Verbosity-level behavior (debug/trace visibility)
//!   2. That something is actually printed in verbose/trace modes
//!   3. That `Printer` forwards messages correctly
//!   4. Quiet-mode behavior

use crate::config;
use crate::logging::*;
use crate::logging::tests::common::*;
use predicates::prelude::*;

mod verbosity_behavior_tests {
    use super::*;

    #[test]
    fn debug_visible_in_verbose() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let err = capture_stderr(|| {
            printer.debug("hello debug");
        });

        assert!(predicates::str::contains("debug").eval(&err));
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

        assert!(predicates::str::contains("trace").eval(&err));
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

        assert!(predicates::str::contains("boom").eval(&err));
    }
}

mod printing_behavior_tests {
    use super::*;

    #[test]
    fn verbose_mode_prints_debug() {
        let printer = make_printer(ModernLogger, LogFormat::Text, Verbosity::Verbose);

        let err = capture_stderr(|| {
            printer.debug("debug message");
        });

        assert!(predicates::str::contains("debug").eval(&err));
    }

    #[test]
    fn trace_mode_prints_trace() {
        let printer = make_printer(ModernLogger, LogFormat::Text, Verbosity::Trace);

        let err = capture_stderr(|| {
            printer.trace("trace message");
        });

        assert!(predicates::str::contains("trace").eval(&err));
    }

    #[test]
    fn quiet_mode_suppresses_non_errors_but_keeps_errors() {
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

        assert!(predicates::str::contains("boom").eval(&err));
    }
}

mod printer_forwarding_tests {
    use super::*;

    #[test]
    fn printer_ok_forwards_simple_logger_output() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer.ok("hello");
        });

        assert!(predicates::str::contains("hello").eval(&out));
    }

    #[test]
    fn printer_warn_forwards_modern_logger_output() {
        let printer = make_printer(ModernLogger, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer.warn("careful");
        });

        assert!(predicates::str::contains("careful").eval(&out));
    }

    #[test_case::test_case(Verbosity::Verbose)]
    #[test_case::test_case(Verbosity::Trace)]
    fn printer_intro_creates_task_in_verbose_like_modes(verbosity: Verbosity) {
        let printer = make_printer(SimpleLogger, LogFormat::Text, verbosity);

        let out = capture_stdout(|| {
            printer.intro("task");
        });

        assert!(predicates::str::contains("task").eval(&out));
    }

    #[test_case::test_case(Verbosity::Verbose)]
    #[test_case::test_case(Verbosity::Trace)]
    fn printer_step_creates_step_in_verbose_like_modes(verbosity: Verbosity) {
        let printer = make_printer(SimpleLogger, LogFormat::Text, verbosity);

        let out = capture_stdout(|| {
            printer.step("processing");
        });

        assert!(predicates::str::contains("processing").eval(&out));
    }
}
