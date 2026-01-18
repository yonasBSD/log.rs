
use crate::logging::*;
use loom::model;

/// Very small loom model to ensure global logger set/get does not panic under interleavings.
/// This is intentionally minimal and can be expanded as the global API grows.
#[test]
fn loom_model_global_logger_set_and_use() {
    model(|| {
        let printer = Printer::new(
            ModernLogger,
            ModernBackend,
            LogFormat::Text,
            Verbosity::Normal,
        );
        crate::logging::set_logger(printer);

        // Simulate a couple of logging calls under the model.
        crate::logging::info("loom-info");
        crate::logging::warn("loom-warn");
    });
}
