use crate::logging::{FormatLogger, Printer, RenderBackend, format_duration};

// -----------------------------------------------------------------------------
// Printer: add dump task tree
// -----------------------------------------------------------------------------
impl<L: FormatLogger, B: RenderBackend> Printer<L, B> {
    pub fn dump_task_tree(&self) {
        if !self.inner.is_verbose() {
            return;
        }

        let tasks = self.tasks.lock().unwrap();
        if tasks.is_empty() {
            println!("(no active tasks)");
            return;
        }

        println!("Active tasks:");
        for (i, t) in tasks.iter().enumerate() {
            let elapsed = t.start.elapsed();
            let timing = format_duration(elapsed);
            println!("  {}. {} (started, +{})", i + 1, t.label, timing);
        }
    }
}
