use crate::logging::tests::common::*;
use crate::logging::*;

use insta::assert_snapshot;

mod nested_span_tests {
    use super::*;

    #[test]
    fn nested_steps_clear_on_outro_snapshot() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("top-level");
            printer.step("first-step");
            printer.step("second-step");
            printer.outro("done");
        });

        assert!(printer.steps.lock().unwrap().is_empty());
        assert!(printer.tasks.lock().unwrap().is_empty());
        assert_snapshot!(out);
    }

    #[test]
    fn nested_tasks_clear_on_outro_snapshot() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("task-1");
            printer.intro("task-2");
            printer.outro("done-2");
            printer.outro("done-1");
        });

        assert!(printer.tasks.lock().unwrap().is_empty());
        assert_snapshot!(out);
    }

    #[test]
    fn dump_tree_shows_active_tasks_snapshot() {
        let printer = make_printer(SimpleLogger, LogFormat::Text, Verbosity::Verbose);

        let out = capture_stdout(|| {
            printer.intro("build");
            printer.intro("test");
            printer.dump_tree();
        });

        assert_snapshot!(out);
    }
}
