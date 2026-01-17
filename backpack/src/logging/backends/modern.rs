use crate::logging::RenderBackend;

/// A backend that renders using cliclack's rich CLI primitives.
pub struct ModernBackend;

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
}
