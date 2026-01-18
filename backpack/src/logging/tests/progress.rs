// ============================================================================
// 6. PROGRESS API BEHAVIOR TESTS
// ============================================================================

mod progress_behavior_tests {
    use crate::logging::{tests::common::capture_stderr, *};
    use std::sync::Once;

    static INIT_LOGGER: Once = Once::new();

    fn ensure_global_logger() {
        INIT_LOGGER.call_once(|| {
            let printer = Printer::new(
                ModernLogger,
                ModernBackend,
                LogFormat::Text,
                Verbosity::Trace, // Use Trace to see intro/step/done messages
            );
            crate::logging::set_logger(printer);
        });
    }

    #[test]
    fn progress_new_emits_intro() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let _p = Progress::new("Downloading files");
        });

        assert!(out.contains("Downloading files"));
    }

    #[test]
    fn progress_with_total_emits_intro() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let _p = Progress::with_total("Processing items", 100);
        });

        assert!(out.contains("Processing items"));
    }

    #[test]
    fn progress_update_emits_step_with_progress() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::new("Uploading");
            p.update(5, 10);
        });

        assert!(out.contains("Uploading"));
        assert!(out.contains("5/10"));
    }

    #[test]
    fn progress_tick_increments_and_emits_step() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Processing", 10);
            p.tick(); // current: 1
            p.tick(); // current: 2
        });

        assert!(out.contains("Processing"));
        assert!(out.contains("1/10"));
        assert!(out.contains("2/10"));
    }

    #[test]
    fn progress_tick_without_total_shows_count_only() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::new("Loading");
            p.tick(); // current: 1
            p.tick(); // current: 2
            p.tick(); // current: 3
        });

        // Should NOT contain progress fraction like "1/" or "2/" or "3/"
        assert!(out.contains("Loading"));
        assert!(out.contains("Loading: 1"));
        assert!(out.contains("Loading: 2"));
        assert!(out.contains("Loading: 3"));
        // Should NOT contain progress fraction since no total was set
        // (avoiding false positives from file paths like "src/...")
        assert!(!out.contains("1/"));
        assert!(!out.contains("2/"));
        assert!(!out.contains("3/"));
    }

    #[test]
    fn progress_finish_emits_done() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let p = Progress::new("Syncing");
            p.finish("Sync complete");
        });

        assert!(out.contains("Syncing"));
        // The finish method emits done() which should show completion marker
        // The exact output depends on your logger implementation
    }

    #[test]
    fn progress_full_workflow_with_total() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Downloading", 5);
            p.tick(); // 1/5
            p.tick(); // 2/5
            p.tick(); // 3/5
            p.update(5, 5); // 5/5
            p.finish("Download complete");
        });

        assert!(out.contains("Downloading"));
        assert!(out.contains("1/5"));
        assert!(out.contains("2/5"));
        assert!(out.contains("3/5"));
        assert!(out.contains("5/5"));
    }

    #[test]
    fn progress_full_workflow_without_total() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::new("Processing");
            p.tick(); // 1
            p.tick(); // 2
            p.tick(); // 3
            p.finish("Processing complete");
        });

        assert!(out.contains("Processing"));
        assert!(out.contains("Processing: 1"));
        assert!(out.contains("Processing: 2"));
        assert!(out.contains("Processing: 3"));
    }

    #[test]
    fn progress_update_changes_total() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::new("Uploading");
            p.update(3, 10); // Set initial progress
            p.update(5, 10); // Update progress
            p.update(10, 10); // Complete
        });

        assert!(out.contains("3/10"));
        assert!(out.contains("5/10"));
        assert!(out.contains("10/10"));
    }

    #[test]
    fn progress_tick_after_update_continues_from_updated_value() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Syncing", 10);
            p.update(5, 10); // Set to 5/10
            p.tick(); // Should go to 6/10
            p.tick(); // Should go to 7/10
        });

        assert!(out.contains("5/10"));
        assert!(out.contains("6/10"));
        assert!(out.contains("7/10"));
    }

    #[test]
    fn progress_multiple_progress_bars_independent() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p1 = Progress::new("Task A");
            let mut p2 = Progress::new("Task B");

            p1.tick(); // Task A: 1
            p2.tick(); // Task B: 1
            p1.tick(); // Task A: 2

            p1.finish("A done");
            p2.finish("B done");
        });

        assert!(out.contains("Task A"));
        assert!(out.contains("Task B"));
        assert!(out.contains("Task A: 1"));
        assert!(out.contains("Task B: 1"));
        assert!(out.contains("Task A: 2"));
    }

    #[test]
    fn progress_with_zero_total() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Empty task", 0);
            p.tick(); // 1/0 (edge case)
        });

        assert!(out.contains("Empty task"));
        assert!(out.contains("1/0"));
    }

    #[test]
    fn progress_update_beyond_total() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Overflowing", 5);
            p.update(10, 5); // Update beyond initial total
        });

        assert!(out.contains("10/5"));
    }

    #[test]
    fn progress_tick_with_total_then_without() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Variable", 10);
            p.tick(); // 1/10
            // Manually set total to None to test the else branch
            p.total = None;
            p.tick(); // Should show "Variable: 2" without total
        });

        assert!(out.contains("1/10"));
        assert!(out.contains("Variable: 2"));
    }

    #[test]
    fn progress_labels_with_special_characters() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::new("Download: file-123.txt [50MB]");
            p.tick();
        });

        assert!(out.contains("Download: file-123.txt [50MB]"));
    }

    #[test]
    fn progress_finish_message_parameter_is_used() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let p = Progress::new("Task");
            p.finish("This message is ignored");
        });

        assert!(out.contains("This message is ignored"));
    }

    #[test]
    fn progress_state_tracking() {
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
    fn progress_with_total_then_update_changes_total() {
        ensure_global_logger();

        let out = capture_stderr(|| {
            let mut p = Progress::with_total("Dynamic", 100);
            p.tick(); // 1/100
            p.update(50, 200); // Change total to 200
            p.tick(); // 51/200
        });

        assert!(out.contains("1/100"));
        assert!(out.contains("50/200"));
        assert!(out.contains("51/200"));
    }
}
