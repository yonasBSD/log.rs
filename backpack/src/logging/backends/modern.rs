use crate::logging::RenderBackend;
use cliclack::ProgressBar;
use std::sync::Mutex;

/// A backend that renders using cliclack's rich CLI primitives.
pub struct ModernBackend {
    bar: std::sync::Mutex<Option<cliclack::ProgressBar>>,
}

impl ModernBackend {
    pub fn new() -> Self {
        Self {
            bar: Mutex::new(None),
        }
    }
}

impl RenderBackend for ModernBackend {
    fn render_error(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::error(msg)?;
        Ok(())
    }

    fn render_info(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::info(msg)?;
        Ok(())
    }

    fn render_remark(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::remark(msg)?;
        Ok(())
    }

    fn render_step(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::step(msg)?;
        Ok(())
    }

    fn render_success(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::success(msg)?;
        Ok(())
    }

    fn render_warning(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::warning(msg)?;
        Ok(())
    }

    fn render_intro(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::intro(msg)?;
        Ok(())
    }

    fn render_outro(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::outro(msg)?;
        Ok(())
    }

    fn render_debug(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::remark(msg)?;
        Ok(())
    }

    fn render_trace(&self, msg: &str) -> anyhow::Result<()> {
        cliclack::log::remark(msg)?;
        Ok(())
    }

    fn render_progress(
        &self,
        label: &str,
        current: u64,
        total: Option<u64>,
        finished: bool,
    ) -> anyhow::Result<()> {
        let mut guard = self.bar.lock().unwrap();

        // Create the bar if needed
        if guard.is_none() {
            let bar = ProgressBar::new(total.unwrap_or(0));
            bar.start(label);
            *guard = Some(bar);
        }

        // Update the bar
        if let Some(bar) = guard.as_ref() {
            // Update total if provided
            if let Some(t) = total {
                bar.set_length(t);
            }

            // Update position
            bar.set_position(current);

            // Update label/message
            bar.set_message(label);

            // Finish if needed
            if finished {
                bar.stop(label);
                *guard = None;
            }
        }

        Ok(())
    }
}
