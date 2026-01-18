mod logger_tests {
    use crate::logging::{tests::common::*, *};

    // ============================================================================
    // TEST FIXTURES
    // ============================================================================

    /// Mock FormatLogger for testing
    ///
    /// This mock lets us exercise the default methods on FormatLogger
    /// without involving any real formatting or I/O.
    struct MockLogger {
        quiet: bool,
        verbose: bool,
    }

    impl MockLogger {
        fn new(verbosity: Verbosity) -> Self {
            Self {
                quiet: verbosity == Verbosity::Quiet,
                verbose: matches!(verbosity, Verbosity::Verbose | Verbosity::Trace),
            }
        }
    }

    impl FormatLogger for MockLogger {
        fn is_quiet(&self) -> bool {
            self.quiet
        }

        fn is_verbose(&self) -> bool {
            self.verbose
        }

        fn ok_raw(&self, m: &str) -> String {
            format!("OK: {}", m)
        }

        fn warn_raw(&self, m: &str) -> String {
            format!("WARN: {}", m)
        }

        fn err_raw(&self, m: &str) -> String {
            format!("ERR: {}", m)
        }

        fn info_raw(&self, m: &str) -> String {
            format!("INFO: {}", m)
        }

        fn dim_raw(&self, m: &str) -> String {
            format!("DIM: {}", m)
        }

        fn intro_raw(&self, m: &str) -> String {
            format!("INTRO: {}", m)
        }

        fn outro_raw(&self, m: &str) -> String {
            format!("OUTRO: {}", m)
        }

        fn done_raw(&self) -> String {
            "DONE!".to_string()
        }

        fn step_raw(&self, m: &str) -> String {
            format!("STEP: {}", m)
        }

        fn debug_raw(&self, m: &str) -> String {
            format!("DEBUG: {}", m)
        }

        fn trace_raw(&self, m: &str) -> String {
            format!("TRACE: {}", m)
        }
    }

    // ============================================================================
    // BASIC ENUM TESTS
    // ============================================================================

    #[test]
    fn test_verbosity_levels() {
        assert_eq!(Verbosity::Quiet, Verbosity::Quiet);
        assert_ne!(Verbosity::Quiet, Verbosity::Normal);
        assert_ne!(Verbosity::Normal, Verbosity::Verbose);
        assert_ne!(Verbosity::Verbose, Verbosity::Trace);
    }

    #[test]
    fn test_log_format_levels() {
        assert_eq!(LogFormat::Text, LogFormat::Text);
        assert_ne!(LogFormat::Text, LogFormat::Json);
    }

    // ============================================================================
    // FORMAT LOGGER TRAIT TESTS
    // ============================================================================

    mod format_logger_tests {
        use super::*;

        #[test]
        fn test_normal_mode_shows_messages() {
            let logger = MockLogger::new(Verbosity::Normal);

            assert_eq!(logger.ok("test"), Some("OK: test".to_string()));
            assert_eq!(logger.warn("test"), Some("WARN: test".to_string()));
            assert_eq!(logger.err("test"), "ERR: test");
            assert_eq!(logger.info("test"), Some("INFO: test".to_string()));
            assert_eq!(logger.dim("test"), Some("DIM: test".to_string()));
            assert_eq!(logger.intro("test"), Some("INTRO: test".to_string()));
            assert_eq!(logger.outro("test"), Some("OUTRO: test".to_string()));
            assert_eq!(logger.done(), Some("DONE!".to_string()));
            assert_eq!(logger.step("test"), Some("STEP: test".to_string()));
        }

        #[test]
        fn test_quiet_mode_suppresses_most_messages() {
            let logger = MockLogger::new(Verbosity::Quiet);

            // Suppressed in quiet mode
            assert_eq!(logger.ok("test"), None);
            assert_eq!(logger.warn("test"), None);
            assert_eq!(logger.info("test"), None);
            assert_eq!(logger.dim("test"), None);
            assert_eq!(logger.intro("test"), None);
            assert_eq!(logger.step("test"), None);
            assert_eq!(logger.debug("test"), None);
            assert_eq!(logger.trace("test"), None);
        }

        #[test]
        fn test_quiet_mode_preserves_outro_done_and_errors() {
            let logger = MockLogger::new(Verbosity::Quiet);

            // Outro and done are not suppressed so quiet builds/tests
            // can still show timing summaries
            assert_eq!(logger.outro("test"), Some("OUTRO: test".to_string()));
            assert_eq!(logger.done(), Some("DONE!".to_string()));

            // Errors are never suppressed
            assert_eq!(logger.err("test"), "ERR: test");
        }

        #[test]
        fn test_verbose_mode_shows_debug_and_trace() {
            let logger = MockLogger::new(Verbosity::Verbose);

            assert_eq!(logger.debug("test"), Some("DEBUG: test".to_string()));
            assert_eq!(logger.trace("test"), Some("TRACE: test".to_string()));
        }

        #[test]
        fn test_normal_mode_hides_debug_and_trace() {
            let logger = MockLogger::new(Verbosity::Normal);

            assert_eq!(logger.debug("test"), None);
            assert_eq!(logger.trace("test"), None);
        }

        #[test]
        fn test_trace_mode_shows_all_verbose_signals() {
            let logger = MockLogger::new(Verbosity::Trace);

            assert!(logger.is_verbose());
            assert_eq!(logger.debug("test"), Some("DEBUG: test".to_string()));
            assert_eq!(logger.trace("test"), Some("TRACE: test".to_string()));
        }
    }

    // ============================================================================
    // SIMPLE LOGGER TESTS
    // ============================================================================

    mod simple_logger_tests {
        use super::*;

        #[test]
        fn test_simple_logger_formats() {
            let logger = SimpleLogger;

            assert!(logger.ok_raw("test").contains("test"));
            assert!(logger.warn_raw("test").contains("test"));
            assert!(logger.err_raw("test").contains("test"));
            assert!(logger.info_raw("test").contains("test"));
        }

        #[test]
        fn test_simple_logger_intro_outro() {
            let logger = SimpleLogger;

            let intro = logger.intro_raw("Starting task");
            assert!(intro.contains("Starting task"));
            assert!(intro.starts_with("→"));

            let outro = logger.outro_raw("Task complete");
            assert!(outro.contains("Task complete"));
            assert!(outro.starts_with("✓"));
        }

        #[test]
        fn test_simple_logger_step() {
            let logger = SimpleLogger;

            let step = logger.step_raw("Processing item");
            assert!(step.contains("Processing item"));
        }
    }

    // ============================================================================
    // MODERN LOGGER TESTS
    // ============================================================================

    mod modern_logger_tests {
        use super::*;

        #[test]
        fn test_modern_logger_formats() {
            let logger = ModernLogger;

            assert!(logger.ok_raw("test").starts_with("✔"));
            assert!(logger.warn_raw("test").starts_with("⚠"));
            assert!(logger.err_raw("test").starts_with("✗"));
            assert!(logger.info_raw("test").starts_with("ℹ"));
            assert!(logger.dim_raw("test").starts_with("›"));
        }

        #[test]
        fn test_modern_logger_task_markers() {
            let logger = ModernLogger;

            assert!(logger.intro_raw("test").starts_with("→"));
            assert!(logger.outro_raw("test").starts_with("✔"));
            assert!(logger.done_raw().starts_with("✔"));
            assert!(logger.step_raw("test").starts_with("⠿"));
        }

        #[test]
        fn test_modern_logger_debug_and_trace_markers() {
            let logger = ModernLogger;

            assert!(logger.debug_raw("test").starts_with("🔍"));
            assert!(logger.trace_raw("test").starts_with("📡"));
        }
    }

    // ============================================================================
    // PRINTER STATE TESTS
    // ============================================================================

    mod printer_tests {
        use super::*;

        #[test]
        fn test_printer_creation() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

            assert_eq!(printer.tasks.lock().unwrap().len(), 0);
            assert_eq!(printer.steps.lock().unwrap().len(), 0);
        }

        #[test]
        fn test_printer_with_json_format() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Json, Verbosity::Normal);

            assert_eq!(printer.format, LogFormat::Json);
        }

        #[test]
        fn test_printer_with_text_format() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

            assert_eq!(printer.format, LogFormat::Text);
        }

        #[test]
        fn test_printer_task_stack_initially_empty() {
            let logger = MockLogger::new(Verbosity::Verbose);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

            assert!(printer.tasks.lock().unwrap().is_empty());
        }

        #[test]
        fn test_printer_step_stack_initially_empty() {
            let logger = MockLogger::new(Verbosity::Verbose);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

            assert!(printer.steps.lock().unwrap().is_empty());
        }
    }

    // ============================================================================
    // EDGE CASE TESTS
    // ============================================================================

    #[test]
    fn test_empty_message() {
        let logger = MockLogger::new(Verbosity::Normal);

        assert_eq!(logger.ok(""), Some("OK: ".to_string()));
        assert_eq!(logger.err(""), "ERR: ");
    }

    #[test]
    fn test_very_long_message() {
        let logger = MockLogger::new(Verbosity::Normal);
        let long_msg = "a".repeat(10000);

        let result = logger.ok(&long_msg);
        assert!(result.is_some());
        assert!(result.unwrap().len() > 10000);
    }

    #[test]
    fn test_verbosity_hierarchy() {
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

    // ============================================================================
    // STRUCTURED FIELDS TESTS
    // ============================================================================

    mod structured_fields_tests {
        use super::*;

        #[test]
        fn json_mode_emits_structured_fields_on_drop() {
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
            let v: serde_json::Value = serde_json::from_str(line).expect("Expected valid JSON");

            assert_eq!(v["message"], "User logged in");
            assert_eq!(v["level"], "info");
            assert_eq!(v["fields"]["user_id"], "42");
            assert_eq!(v["fields"]["role"], "admin");
        }

        #[test]
        fn text_mode_emits_structured_fields_on_drop() {
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
        fn text_mode_emits_fields_for_ok_event() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

            let out = capture_stdout(|| {
                printer
                    .ok_event("Connected to database")
                    .field("host", "localhost")
                    .field("port", 5432);
            });

            assert!(out.contains("Connected to database"));
            assert!(out.contains("host=localhost"));
            assert!(out.contains("port=5432"));
        }

        #[test]
        fn text_mode_emits_fields_for_warn_event() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

            let out = capture_stdout(|| {
                printer
                    .warn_event("Retrying connection")
                    .field("attempt", 3)
                    .field("max_attempts", 5);
            });

            assert!(out.contains("Retrying connection"));
            assert!(out.contains("attempt=3"));
            assert!(out.contains("max_attempts=5"));
        }

        #[test]
        fn text_mode_emits_fields_for_error_event() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

            let out = capture_stderr(|| {
                printer
                    .err_event("Connection failed")
                    .field("server", "smtp.example.com")
                    .field("error_code", 500);
            });

            assert!(out.contains("Connection failed"));
            assert!(out.contains("server=smtp.example.com"));
            assert!(out.contains("error_code=500"));
        }

        #[test]
        fn text_mode_emits_multiple_fields() {
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

            assert!(out.contains("Batch processing complete"));
            assert!(out.contains("processed=1250"));
            assert!(out.contains("failed=23"));
            assert!(out.contains("skipped=5"));
            assert!(out.contains("duration_ms=3456"));
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
        fn text_mode_emits_fields_for_debug_in_verbose_mode() {
            let logger = MockLogger::new(Verbosity::Verbose);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Verbose);

            let out = capture_stderr(|| {
                printer
                    .debug_event("Request processed")
                    .field("duration_ms", 145)
                    .field("cache_hit", true);
            });

            assert!(out.contains("Request processed"));
            assert!(out.contains("duration_ms=145"));
            assert!(out.contains("cache_hit=true"));
        }

        #[test]
        fn text_mode_emits_fields_for_trace_in_trace_mode() {
            let logger = MockLogger::new(Verbosity::Trace);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Trace);

            let out = capture_stderr(|| {
                printer
                    .trace_event("SQL query executed")
                    .field("query", "SELECT * FROM users")
                    .field("execution_time_ms", 12);
            });

            assert!(out.contains("SQL query executed"));
            assert!(out.contains("query=SELECT * FROM users"));
            assert!(out.contains("execution_time_ms=12"));
        }

        #[test]
        fn text_mode_handles_string_fields() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

            let out = capture_stdout(|| {
                printer
                    .ok_event("Deployment successful")
                    .field("region", "us-east-1")
                    .field("environment", "production");
            });

            assert!(out.contains("Deployment successful"));
            assert!(out.contains("region=us-east-1"));
            assert!(out.contains("environment=production"));
        }

        #[test]
        fn text_mode_handles_numeric_fields() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

            let out = capture_stdout(|| {
                printer
                    .info("Metrics reported")
                    .field("count", 234)
                    .field("cpu_percent", 23)
                    .field("memory_gb", 1.2);
            });

            assert!(out.contains("Metrics reported"));
            assert!(out.contains("count=234"));
            assert!(out.contains("cpu_percent=23"));
            assert!(out.contains("memory_gb=1.2"));
        }

        #[test]
        fn text_mode_fields_work_with_step_event() {
            let logger = MockLogger::new(Verbosity::Trace);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Trace);

            let out = capture_stderr(|| {
                printer
                    .step_event("Uploading files")
                    .field("count", 203)
                    .field("cdn", "cdn.example.com");
            });

            assert!(out.contains("Uploading files"));
            assert!(out.contains("count=203"));
            assert!(out.contains("cdn=cdn.example.com"));
        }

        #[test]
        fn text_mode_fields_work_with_intro_event() {
            let logger = MockLogger::new(Verbosity::Trace);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Trace);

            let out = capture_stderr(|| {
                printer
                    .intro_event("Starting deployment")
                    .field("version", "1.2.3")
                    .field("target", "production");
            });

            assert!(out.contains("Starting deployment"));
            assert!(out.contains("version=1.2.3"));
            assert!(out.contains("target=production"));
        }

        #[test]
        fn text_mode_fields_work_with_outro_event() {
            let logger = MockLogger::new(Verbosity::Trace);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Trace);

            let out = capture_stderr(|| {
                printer
                    .outro_event("Deployment complete")
                    .field("duration_seconds", 45)
                    .field("status", "success");
            });

            assert!(out.contains("Deployment complete"));
            assert!(out.contains("duration_seconds=45"));
            assert!(out.contains("status=success"));
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
        fn text_and_json_modes_both_handle_fields() {
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

            // Both modes include the message
            assert!(text_out.contains("Task completed"));
            assert!(json_out.contains("Task completed"));

            // Text mode uses key=value format
            assert!(text_out.contains("items=100"));
            assert!(text_out.contains("errors=0"));

            // JSON mode uses structured fields
            let line = json_out.lines().find(|l| !l.trim().is_empty()).unwrap();
            let v: serde_json::Value = serde_json::from_str(line).unwrap();
            assert_eq!(v["fields"]["items"], "100");
            assert_eq!(v["fields"]["errors"], "0");
        }
    }

    // ============================================================================
    // ROADMAP FEATURE PLACEHOLDERS (ignored until implemented)
    // ============================================================================
    mod roadmap_feature_tests {
        #[test]
        #[ignore]
        fn plugin_system_not_yet_implemented() {
            // Placeholder for future plugin system tests.
            // Expected: ability to register custom formatters/backends.
            assert!(true);
        }

        #[test]
        #[ignore]
        fn compile_time_log_level_stripping_not_yet_implemented() {
            // Placeholder for future compile-time stripping tests.
            assert!(true);
        }

        #[test]
        #[ignore]
        fn log_capture_api_not_yet_implemented() {
            // Placeholder for future log capture API tests.
            assert!(true);
        }

        #[test]
        #[ignore]
        fn opentelemetry_integration_not_yet_implemented() {
            // Placeholder for future OpenTelemetry integration tests.
            assert!(true);
        }

        #[test]
        #[ignore]
        fn sampling_not_yet_implemented() {
            // Placeholder for future sampling tests.
            assert!(true);
        }
    }
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use crate::logging::*;

    #[test]
    fn test_simple_logger_workflow() {
        let logger = SimpleLogger;

        let intro = logger.intro_raw("Starting deployment");
        assert!(intro.contains("Starting deployment"));

        let step1 = logger.step_raw("Building assets");
        assert!(step1.contains("Building assets"));

        let step2 = logger.step_raw("Uploading files");
        assert!(step2.contains("Uploading files"));

        let outro = logger.outro_raw("Deployment complete");
        assert!(outro.contains("Deployment complete"));
    }

    #[test]
    fn test_modern_logger_workflow() {
        let logger = ModernLogger;

        let intro = logger.intro_raw("Running tests");
        let step = logger.step_raw("Test suite 1");
        let ok = logger.ok_raw("All tests passed");
        let outro = logger.outro_raw("Testing complete");

        assert!(intro.starts_with("→"));
        assert!(step.starts_with("⠿"));
        assert!(ok.starts_with("✔"));
        assert!(outro.starts_with("✔"));
    }

    #[test]
    fn test_error_always_visible() {
        let logger = SimpleLogger;

        let err1 = logger.err_raw("Critical error");
        let err2 = logger.err_raw("Critical error");

        assert_eq!(err1, err2);
        assert!(err1.contains("Critical error"));
    }
}
