
use crate::logging::{tests::common::capture_stderr, *};
use insta::assert_snapshot;
use serial_test::serial;
use std::sync::Once;

static INIT_LOGGER: Once = Once::new();

fn ensure_global_logger() {
    INIT_LOGGER.call_once(|| {
        let printer = Printer::new(
            ModernLogger,
            ModernBackend,
            LogFormat::Text,
            Verbosity::Trace,
        );
        crate::logging::set_logger(printer);
    });
}

mod progress_behavior_tests {
    use super::*;

    #[test]
    #[serial]
    fn progress_new_emits_intro_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let _p = Progress::new("Downloading files");
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_with_total_emits_intro_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let _p = Progress::with_total("Processing items", 100);
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_update_emits_step_with_progress_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::new("Uploading");
            p.update(5, 10);
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_tick_increments_and_emits_step_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Processing", 10);
            p.tick();
            p.tick();
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_tick_without_total_shows_count_only_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::new("Loading");
            p.tick();
            p.tick();
            p.tick();
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_finish_emits_done_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let p = Progress::new("Syncing");
            p.finish("Sync complete");
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_full_workflow_with_total_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Downloading", 5);
            p.tick();
            p.tick();
            p.tick();
            p.update(5, 5);
            p.finish("Download complete");
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_full_workflow_without_total_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::new("Processing");
            p.tick();
            p.tick();
            p.tick();
            p.finish("Processing complete");
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_update_changes_total_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::new("Uploading");
            p.update(3, 10);
            p.update(5, 10);
            p.update(10, 10);
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_tick_after_update_continues_from_updated_value_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Syncing", 10);
            p.update(5, 10);
            p.tick();
            p.tick();
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_multiple_progress_bars_independent_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p1 = Progress::new("Task A");
            let mut p2 = Progress::new("Task B");

            p1.tick();
            p2.tick();
            p1.tick();

            p1.finish("A done");
            p2.finish("B done");
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_with_zero_total_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Empty task", 0);
            p.tick();
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_update_beyond_total_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Overflowing", 5);
            p.update(10, 5);
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_tick_with_total_then_without_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Variable", 10);
            p.tick();
            p.total = None;
            p.tick();
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_labels_with_special_characters_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::new("Download: file-123.txt [50MB]");
            p.tick();
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_finish_message_parameter_is_used_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let p = Progress::new("Task");
            p.finish("This message is ignored");
        });

        assert_snapshot!(out);
    }

    #[test]
    #[serial]
    fn progress_state_tracking_internal_fields() {
        ensure_global_logger();

        let mut p = Progress::new("State test");
        assert_eq!(p.current, 0);
        assert_eq!(p.total, None);

        p.tick();
        assert_eq!(p.current, 1);

        p.update(5, 10);
        assert_eq!(p.current, 5);
        assert_eq!(p.total, Some(10));

        p.tick();
        assert_eq!(p.current, 6);
    }

    #[test]
    #[serial]
    fn progress_with_total_then_update_changes_total_snapshot() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Dynamic", 100);
            p.tick();
            p.update(50, 200);
            p.tick();
        });

        assert_snapshot!(out);
    }
}

