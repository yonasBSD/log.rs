
use crate::config;
use crate::logging::*;
use crate::logging::tests::common::*;

use insta::assert_snapshot;
use std::time::Duration;

mod timing_tests {
    use super::*;

    #[test]
    fn outro_includes_timing_in_verbose_mode_snapshot() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("timed-task");
            std::thread::sleep(Duration::from_millis(20));
            printer.outro("finished");
        });

        assert_snapshot!(out);
    }

    #[test]
    fn nested_timing_tracks_independently_snapshot() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("outer");
            std::thread::sleep(Duration::from_millis(10));

            printer.intro("inner");
            std::thread::sleep(Duration::from_millis(10));
            printer.outro("inner-done");

            printer.outro("outer-done");
        });

        assert_snapshot!(out);
    }

    #[test]
    fn quiet_mode_preserves_timing_summaries_snapshot() {
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

        assert_snapshot!(out);
    }
}

