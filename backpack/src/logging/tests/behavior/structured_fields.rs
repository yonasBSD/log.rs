use crate::logging::tests::common::*;
use crate::logging::*;

use insta::assert_snapshot;

mod structured_fields_tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn json_mode_emits_structured_fields_on_drop_snapshot() {
        let logger = MockLogger::new(Verbosity::Normal);
        let printer = Printer::new(logger, SimpleBackend, LogFormat::Json, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer
                .info("User logged in")
                .field("user_id", 42)
                .field("role", "admin");
        });

        let line = out
            .lines()
            .find(|l| !l.trim().is_empty())
            .expect("Expected output");
        let v: Value = serde_json::from_str(line).expect("Expected valid JSON");

        assert_eq!(v["message"], "User logged in");
        assert_eq!(v["level"], "info");
        assert_eq!(v["fields"]["user_id"], "42");
        assert_eq!(v["fields"]["role"], "admin");
        assert_snapshot!(out);
    }

    #[test]
    fn text_mode_emits_structured_fields_on_drop_snapshot() {
        let logger = MockLogger::new(Verbosity::Normal);
        let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer
                .info("User logged in")
                .field("user_id", 42)
                .field("role", "admin");
        });

        assert!(out.contains("User logged in"));
        assert!(out.contains("user_id=42"));
        assert!(out.contains("role=admin"));
        assert_snapshot!(out);
    }

    #[test]
    fn text_mode_does_not_include_json_structure() {
        let logger = MockLogger::new(Verbosity::Normal);
        let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer
                .info("User logged in")
                .field("user_id", 42)
                .field("role", "admin");
        });

        assert!(!out.contains("\"fields\""));
    }

    #[test]
    fn text_mode_emits_fields_for_ok_warn_err_events() {
        let logger = MockLogger::new(Verbosity::Normal);
        let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

        let ok_out = capture_stdout(|| {
            printer
                .ok_event("Connected to database")
                .field("host", "localhost")
                .field("port", 5432);
        });

        let warn_out = capture_stdout(|| {
            printer
                .warn_event("Retrying connection")
                .field("attempt", 3)
                .field("max_attempts", 5);
        });

        let err_out = capture_stderr(|| {
            printer
                .err_event("Connection failed")
                .field("server", "smtp.example.com")
                .field("error_code", 500);
        });

        assert_snapshot!(ok_out);
        assert_snapshot!(warn_out);
        assert_snapshot!(err_out);
    }

    #[test]
    fn text_mode_emits_multiple_fields_snapshot() {
        let logger = MockLogger::new(Verbosity::Normal);
        let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer
                .info("Batch processing complete")
                .field("processed", 1250)
                .field("failed", 23)
                .field("skipped", 5)
                .field("duration_ms", 3456);
        });

        assert_snapshot!(out);
    }

    #[test]
    fn text_mode_handles_empty_fields() {
        let logger = MockLogger::new(Verbosity::Normal);
        let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer.info("Simple message");
        });

        assert!(out.contains("Simple message"));
        assert!(!out.contains("="));
    }

    #[test]
    fn text_mode_emits_fields_for_debug_and_trace_in_verbose_modes() {
        let logger = MockLogger::new(Verbosity::Verbose);
        let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Verbose);

        let debug_out = capture_stderr(|| {
            printer
                .debug_event("Request processed")
                .field("duration_ms", 145)
                .field("cache_hit", true);
        });

        let trace_logger = MockLogger::new(Verbosity::Trace);
        let trace_printer = Printer::new(
            trace_logger,
            SimpleBackend,
            LogFormat::Text,
            Verbosity::Trace,
        );

        let trace_out = capture_stderr(|| {
            trace_printer
                .trace_event("SQL query executed")
                .field("query", "SELECT * FROM users")
                .field("execution_time_ms", 12);
        });

        assert_snapshot!(debug_out);
        assert_snapshot!(trace_out);
    }

    #[test]
    fn text_mode_handles_string_and_numeric_fields_snapshot() {
        let logger = MockLogger::new(Verbosity::Normal);
        let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer
                .info("Metrics reported")
                .field("count", 234)
                .field("cpu_percent", 23)
                .field("memory_gb", 1.2);
        });

        assert_snapshot!(out);
    }

    #[test]
    fn text_mode_fields_respect_quiet_mode() {
        let logger = MockLogger::new(Verbosity::Quiet);
        let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Quiet);

        let out = capture_stdout(|| {
            printer
                .info("This should be suppressed")
                .field("user_id", 42);
        });

        assert!(!out.contains("This should be suppressed"));
        assert!(!out.contains("user_id=42"));
    }

    #[test]
    fn text_and_json_modes_both_handle_fields_snapshot() {
        let logger_text = MockLogger::new(Verbosity::Normal);
        let printer_text = Printer::new(
            logger_text,
            SimpleBackend,
            LogFormat::Text,
            Verbosity::Normal,
        );

        let logger_json = MockLogger::new(Verbosity::Normal);
        let printer_json = Printer::new(
            logger_json,
            SimpleBackend,
            LogFormat::Json,
            Verbosity::Normal,
        );

        let text_out = capture_stdout(|| {
            printer_text
                .ok_event("Task completed")
                .field("items", 100)
                .field("errors", 0);
        });

        let json_out = capture_stdout(|| {
            printer_json
                .ok_event("Task completed")
                .field("items", 100)
                .field("errors", 0);
        });

        assert_snapshot!("text_mode_ok_event_fields", text_out);
        assert_snapshot!("json_mode_ok_event_fields", json_out);
    }

    #[test]
    fn json_mode_structured_fields_via_drop_snapshot() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer
                .info("User logged in")
                .field("user_id", 7)
                .field("role", "admin");
        });

        assert_snapshot!(out);
    }

    #[test]
    fn text_mode_structured_fields_via_drop_snapshot() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer
                .info("User logged in")
                .field("user_id", 7)
                .field("role", "admin");
        });

        assert_snapshot!(out);
    }
}
