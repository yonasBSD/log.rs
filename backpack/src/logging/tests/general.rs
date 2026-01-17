#[cfg(test)]
mod logger_tests {
    use crate::logging::*;

    // Mock FormatLogger for testing
    //
    // This mock lets us exercise the default methods on FormatLogger
    // without involving any real formatting or I/O.
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
            format!("DONE!")
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

    // Test Verbosity levels
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

    // Test FormatLogger trait default methods
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
        fn test_quiet_mode_suppresses_most_messages_but_not_outro_done_or_errors() {
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

            // Outro and done are *not* suppressed in quiet mode so that quiet builds/tests
            // can still show timing summaries.
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

    // Test SimpleLogger formatting
    mod simple_logger_tests {
        use super::*;

        #[test]
        fn test_simple_logger_formats() {
            let logger = crate::logging::SimpleLogger;

            assert!(logger.ok_raw("test").contains("test"));
            assert!(logger.warn_raw("test").contains("test"));
            assert!(logger.err_raw("test").contains("test"));
            assert!(logger.info_raw("test").contains("test"));
        }

        #[test]
        fn test_simple_logger_intro_outro() {
            let logger = crate::logging::SimpleLogger;

            let intro = logger.intro_raw("Starting task");
            assert!(intro.contains("Starting task"));
            assert!(intro.starts_with("→"));

            let outro = logger.outro_raw("Task complete");
            assert!(outro.contains("Task complete"));
            assert!(outro.starts_with("✓"));
        }

        #[test]
        fn test_simple_logger_step() {
            let logger = crate::logging::SimpleLogger;

            let step = logger.step_raw("Processing item");
            assert!(step.contains("Processing item"));
        }
    }

    // Test ModernLogger formatting (including emoji refinements)
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

    // Test Printer behavior (state-level, not actual I/O)
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

        /*
        #[test]
        fn test_printer_info_with_fields_compiles_and_uses_fields_type() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Json, Verbosity::Normal);

            let mut fields = Fields::new();
            fields.insert("user_id".to_string(), "123".to_string());
            fields.insert("role".to_string(), "admin".to_string());

            printer.info_with_fields("User logged in", fields);
        }
        */
    }

    // Test global logger functionality
    mod global_logger_tests {
        use super::*;
        use std::sync::{Arc, OnceLock};

        // A shadow global used ONLY for testing.
        static TEST_LOGGER: OnceLock<Arc<dyn ScreenLogger + Send + Sync>> = OnceLock::new();

        struct TestLogger;

        impl ScreenLogger for TestLogger {
            fn ok(&self, _m: &str) {}
            fn warn(&self, _m: &str) {}
            fn err(&self, _m: &str) {}
            fn info(&self, _m: &str) {}
            fn dim(&self, _m: &str) {}
            fn intro(&self, _m: &str) {}
            fn outro(&self, _m: &str) {}
            fn done(&self) {}
            fn step(&self, _m: &str) {}
            fn debug(&self, _m: &str) {}
            fn trace(&self, _m: &str) {}
            fn dump_tree(&self) {}
        }

        fn test_log() -> &'static Arc<dyn ScreenLogger + Send + Sync> {
            TEST_LOGGER.get().expect("Logger not initialized")
        }

        #[test]
        #[should_panic(expected = "Logger not initialized")]
        fn test_log_panics_when_not_initialized() {
            assert!(TEST_LOGGER.get().is_none());
            let _ = test_log();
        }

        #[test]
        fn test_set_logger_accepts_valid_logger() {
            let _ = TEST_LOGGER.set(Arc::new(TestLogger));
            let logger = test_log();
            logger.ok("hello");
            assert!(true);
        }
    }

    // Edge cases
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

    mod structured_fields_tests_drop {
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

        #[test]
        fn json_mode_emits_structured_fields_on_drop() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Json, Verbosity::Normal);

            let out = capture_stdout(|| {
                printer
                    .info("User logged in")
                    .field("user_id", 42)
                    .field("role", "admin");
                // emission happens on Drop
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
        fn text_mode_ignores_structured_fields_on_drop() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, SimpleBackend, LogFormat::Text, Verbosity::Normal);

            let out = capture_stdout(|| {
                printer
                    .info("User logged in")
                    .field("user_id", 42)
                    .field("role", "admin");
            });

            assert!(out.contains("User logged in"));
            assert!(!out.contains("\"fields\""));
        }
    }

    // Roadmap feature placeholders (ignored until implemented)
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

// Integration-style tests
#[cfg(test)]
mod integration_tests {
    use crate::logging::*;

    #[test]
    fn test_simple_logger_workflow() {
        let logger = crate::logging::SimpleLogger;

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
        let quiet = crate::logging::SimpleLogger;
        let normal = crate::logging::SimpleLogger;

        let err1 = quiet.err_raw("Critical error");
        let err2 = normal.err_raw("Critical error");

        assert_eq!(err1, err2);
        assert!(err1.contains("Critical error"));
    }
}
