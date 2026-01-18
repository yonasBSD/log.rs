
mod json_format_behavior_tests {
    //use super::*;
    use crate::logging::*;
    use crate::logging::tests::common::*;

    use insta::assert_snapshot;
    use pretty_assertions::assert_eq;
    use serde_json::Value;

    #[test]
    fn json_mode_produces_valid_json_snapshot() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Normal);

        let out = capture_stdout(|| {
            printer.ok("hello");
        });

        assert_snapshot!(out);
        for line in out.lines().filter(|l| !l.trim().is_empty()) {
            serde_json::from_str::<Value>(line).expect("Expected valid JSON output");
        }
    }

    #[test]
    fn json_mode_errors_are_valid_json_snapshot() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Quiet);

        let out = capture_stderr(|| {
            printer.err("boom");
        });

        assert_snapshot!(out);
        for line in out.lines().filter(|l| !l.trim().is_empty()) {
            serde_json::from_str::<Value>(line).expect("Expected valid JSON output");
        }
    }

    #[test]
    fn json_mode_outputs_json_for_spans_snapshot() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Trace);

        let out = capture_stdout(|| {
            printer.intro("task");
            printer.step("step");
            printer.outro("done");
        });

        assert_snapshot!(out);
        for line in out.lines().filter(|l| !l.trim().is_empty()) {
            serde_json::from_str::<Value>(line).expect("Expected valid JSON output");
        }
    }

    #[test]
    fn json_mode_includes_structured_fields_snapshot() {
        let printer = make_printer(SimpleLogger, LogFormat::Json, Verbosity::Normal);

        let out = capture_stdout(|| {
            let mut fields = Fields::new();
            fields.insert("user_id".to_string(), "42".to_string());
            fields.insert("role".to_string(), "admin".to_string());
            printer.info_with_fields("User logged in", &fields);
        });

        let line = out
            .lines()
            .find(|l| !l.trim().is_empty())
            .expect("Expected output");
        let v: serde_json::Value = serde_json::from_str(line).expect("Expected valid JSON");

        assert_eq!(v["message"], "User logged in");
        assert_eq!(v["fields"]["user_id"], "42");
        assert_eq!(v["fields"]["role"], "admin");
        assert_snapshot!(out);
    }
}

