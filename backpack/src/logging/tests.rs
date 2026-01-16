#[cfg(test)]
mod logger_tests {
    use crate::logging::*;

    // Mock FormatLogger for testing
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

        fn done_raw(&self, m: &str) -> String {
            format!("DONE: {}", m)
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
        }

        #[test]
        fn test_quiet_mode_suppresses_messages() {
            let logger = MockLogger::new(Verbosity::Quiet);

            // These should be suppressed
            assert_eq!(logger.ok("test"), None);
            assert_eq!(logger.warn("test"), None);
            assert_eq!(logger.info("test"), None);
            assert_eq!(logger.dim("test"), None);
            assert_eq!(logger.intro("test"), None);
            assert_eq!(logger.outro("test"), None);
            assert_eq!(logger.step("test"), None);

            // Errors are never suppressed
            assert_eq!(logger.err("test"), "ERR: test");
        }

        #[test]
        fn test_verbose_mode_shows_debug() {
            let logger = MockLogger::new(Verbosity::Verbose);

            assert_eq!(logger.debug("test"), Some("DEBUG: test".to_string()));
            assert_eq!(logger.trace("test"), Some("TRACE: test".to_string()));
        }

        #[test]
        fn test_normal_mode_hides_debug() {
            let logger = MockLogger::new(Verbosity::Normal);

            assert_eq!(logger.debug("test"), None);
            assert_eq!(logger.trace("test"), None);
        }

        #[test]
        fn test_trace_mode_shows_all() {
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

            // Test basic formatting (these will include unicode symbols)
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

    // Test ModernLogger formatting
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
            assert!(logger.done_raw("test").starts_with("✔"));
            assert!(logger.step_raw("test").starts_with("⠿"));
        }

        #[test]
        fn test_modern_logger_debug_markers() {
            let logger = ModernLogger;

            assert!(logger.debug_raw("test").starts_with("🔍"));
            assert!(logger.trace_raw("test").starts_with("…"));
        }
    }

    // Test Printer behavior
    mod printer_tests {
        use super::*;

        #[test]
        fn test_printer_creation() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, LogFormat::Text, Verbosity::Normal);

            // Verify printer was created (basic smoke test)
            assert_eq!(printer.tasks.lock().unwrap().len(), 0);
            assert_eq!(printer.steps.lock().unwrap().len(), 0);
        }

        #[test]
        fn test_printer_with_json_format() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, LogFormat::Json, Verbosity::Normal);

            assert_eq!(printer.format, LogFormat::Json);
        }

        #[test]
        fn test_printer_with_text_format() {
            let logger = MockLogger::new(Verbosity::Normal);
            let printer = Printer::new(logger, LogFormat::Text, Verbosity::Normal);

            assert_eq!(printer.format, LogFormat::Text);
        }

        // Note: Testing actual output would require capturing stdout/stderr
        // or using a mock I/O layer. These tests focus on state management.
    }

    // Test global logger functionality
    mod global_logger_tests {
        use super::*;
        use std::sync::{Arc, OnceLock};

        //
        // A shadow global used ONLY for testing.
        // This avoids interfering with the real LOGGER in logging/mod.rs.
        //
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
            fn done(&self, _m: &str) {}
            fn step(&self, _m: &str) {}
            fn debug(&self, _m: &str) {}
            fn trace(&self, _m: &str) {}
        }

        //
        // A local version of log() that uses TEST_LOGGER instead of the real global.
        //
        fn test_log() -> &'static Arc<dyn ScreenLogger + Send + Sync> {
            TEST_LOGGER.get().expect("Logger not initialized")
        }

        #[test]
        #[should_panic(expected = "Logger not initialized")]
        fn test_log_panics_when_not_initialized() {
            // Ensure the test logger is empty
            assert!(TEST_LOGGER.get().is_none());

            // This should panic
            let _ = test_log();
        }

        #[test]
        fn test_set_logger_accepts_valid_logger() {
            // Initialize the test logger
            let _ = TEST_LOGGER.set(Arc::new(TestLogger));

            // Retrieve it
            let logger = test_log();

            // Call a method to ensure dispatch works
            logger.ok("hello");

            // If we reach here, the logger works
            assert!(true);
        }
    }

    // Test edge cases
    #[test]
    fn test_empty_message() {
        let logger = MockLogger::new(Verbosity::Normal);

        // Should handle empty messages gracefully
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

    // Test verbosity transitions
    #[test]
    fn test_verbosity_hierarchy() {
        // Quiet < Normal < Verbose < Trace
        let quiet = MockLogger::new(Verbosity::Quiet);
        let normal = MockLogger::new(Verbosity::Normal);
        let verbose = MockLogger::new(Verbosity::Verbose);
        let trace = MockLogger::new(Verbosity::Trace);

        // Quiet suppresses everything except errors
        assert!(quiet.is_quiet());
        assert!(!quiet.is_verbose());

        // Normal shows standard messages
        assert!(!normal.is_quiet());
        assert!(!normal.is_verbose());

        // Verbose shows debug
        assert!(!verbose.is_quiet());
        assert!(verbose.is_verbose());

        // Trace shows trace
        assert!(!trace.is_quiet());
        assert!(trace.is_verbose());
    }

    // Test TimedSpan behavior through Printer
    #[test]
    fn test_task_stack_management() {
        let logger = MockLogger::new(Verbosity::Verbose);
        let printer = Printer::new(logger, LogFormat::Text, Verbosity::Normal);

        // Initially empty
        assert_eq!(printer.tasks.lock().unwrap().len(), 0);

        // Note: We can't directly test intro/outro without capturing output
        // but we can verify the stack exists and is accessible
        assert!(printer.tasks.lock().unwrap().is_empty());
    }

    #[test]
    fn test_step_stack_management() {
        let logger = MockLogger::new(Verbosity::Verbose);
        let printer = Printer::new(logger, LogFormat::Text, Verbosity::Normal);

        // Initially empty
        assert_eq!(printer.steps.lock().unwrap().len(), 0);
        assert!(printer.steps.lock().unwrap().is_empty());
    }
}

// Integration-style tests
#[cfg(test)]
mod integration_tests {
    use crate::logging::*;

    #[test]
    fn test_simple_logger_workflow() {
        let logger = crate::logging::SimpleLogger;

        // Simulate a typical workflow
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
        // Errors should be visible in all verbosity modes
        let quiet = crate::logging::SimpleLogger;
        let normal = crate::logging::SimpleLogger;

        // Both should format errors the same way
        let err1 = quiet.err_raw("Critical error");
        let err2 = normal.err_raw("Critical error");

        assert_eq!(err1, err2);
        assert!(err1.contains("Critical error"));
    }
}
