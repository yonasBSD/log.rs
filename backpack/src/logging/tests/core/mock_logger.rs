use crate::logging::*;
use crate::logging::tests::common::*;

use proptest::prelude::*;
use insta::assert_snapshot;

mod format_logger_default_behavior {
    use super::*;

    #[test]
    fn normal_mode_shows_all_non_verbose_messages() {
        let logger = MockLogger::new(Verbosity::Normal);

        pretty_assertions::assert_eq!(logger.ok("test"), Some("OK: test".to_string()));
        pretty_assertions::assert_eq!(logger.warn("test"), Some("WARN: test".to_string()));
        pretty_assertions::assert_eq!(logger.err("test"), "ERR: test");
        pretty_assertions::assert_eq!(logger.info("test"), Some("INFO: test".to_string()));
        pretty_assertions::assert_eq!(logger.dim("test"), Some("DIM: test".to_string()));
        pretty_assertions::assert_eq!(logger.intro("test"), Some("INTRO: test".to_string()));
        pretty_assertions::assert_eq!(logger.outro("test"), Some("OUTRO: test".to_string()));
        pretty_assertions::assert_eq!(logger.done(), Some("DONE!".to_string()));
        pretty_assertions::assert_eq!(logger.step("test"), Some("STEP: test".to_string()));
    }

    #[test]
    fn quiet_mode_suppresses_most_messages() {
        let logger = MockLogger::new(Verbosity::Quiet);

        pretty_assertions::assert_eq!(logger.ok("test"), None);
        pretty_assertions::assert_eq!(logger.warn("test"), None);
        pretty_assertions::assert_eq!(logger.info("test"), None);
        pretty_assertions::assert_eq!(logger.dim("test"), None);
        pretty_assertions::assert_eq!(logger.intro("test"), None);
        pretty_assertions::assert_eq!(logger.step("test"), None);
        pretty_assertions::assert_eq!(logger.debug("test"), None);
        pretty_assertions::assert_eq!(logger.trace("test"), None);
    }

    #[test]
    fn quiet_mode_preserves_outro_done_and_errors() {
        let logger = MockLogger::new(Verbosity::Quiet);

        pretty_assertions::assert_eq!(logger.outro("test"), Some("OUTRO: test".to_string()));
        pretty_assertions::assert_eq!(logger.done(), Some("DONE!".to_string()));
        pretty_assertions::assert_eq!(logger.err("test"), "ERR: test");
    }

    #[test]
    fn verbose_mode_shows_debug_and_trace() {
        let logger = MockLogger::new(Verbosity::Verbose);

        pretty_assertions::assert_eq!(logger.debug("test"), Some("DEBUG: test".to_string()));
        pretty_assertions::assert_eq!(logger.trace("test"), Some("TRACE: test".to_string()));
    }

    #[test]
    fn normal_mode_hides_debug_and_trace() {
        let logger = MockLogger::new(Verbosity::Normal);

        pretty_assertions::assert_eq!(logger.debug("test"), None);
        pretty_assertions::assert_eq!(logger.trace("test"), None);
    }

    #[test]
    fn trace_mode_shows_all_verbose_signals() {
        let logger = MockLogger::new(Verbosity::Trace);

        assert!(logger.is_verbose());
        pretty_assertions::assert_eq!(logger.debug("test"), Some("DEBUG: test".to_string()));
        pretty_assertions::assert_eq!(logger.trace("test"), Some("TRACE: test".to_string()));
    }

    #[test]
    fn empty_message_behavior() {
        let logger = MockLogger::new(Verbosity::Normal);

        pretty_assertions::assert_eq!(logger.ok(""), Some("OK: ".to_string()));
        pretty_assertions::assert_eq!(logger.err(""), "ERR: ");
    }

    proptest! {
        #[test]
        fn ok_includes_arbitrary_message(s in ".*") {
            let logger = MockLogger::new(Verbosity::Normal);
            let out = logger.ok(&s).unwrap();
            prop_assert!(out.contains(&s));
        }

        #[test]
        fn err_includes_arbitrary_message(s in ".*") {
            let logger = MockLogger::new(Verbosity::Normal);
            let out = logger.err(&s);
            prop_assert!(out.contains(&s));
        }
    }

    #[test_case::test_case(Verbosity::Quiet)]
    #[test_case::test_case(Verbosity::Normal)]
    #[test_case::test_case(Verbosity::Verbose)]
    #[test_case::test_case(Verbosity::Trace)]
    fn snapshot_ok_output_across_verbosity(verbosity: Verbosity) {
        let logger = MockLogger::new(verbosity);
        let out = logger.ok("snapshot-test");
        assert_snapshot!(format!("{:?}", (verbosity, out)), &format!("verbosity_{verbosity:?}"));
    }
}
