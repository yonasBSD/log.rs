use crate::logging::RenderBackend;

/// A simple backend that renders to stdout/stderr.
pub struct SimpleBackend;

impl RenderBackend for SimpleBackend {
    fn render_error(&self, msg: &str) -> anyhow::Result<()> {
        eprintln!("{msg}");
        Ok(())
    }

    fn render_info(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_remark(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_step(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_success(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_warning(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_intro(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_outro(&self, msg: &str) -> anyhow::Result<()> {
        println!("{msg}");
        Ok(())
    }

    fn render_debug(&self, msg: &str) -> anyhow::Result<()> {
        eprintln!("{msg}");
        Ok(())
    }

    fn render_trace(&self, msg: &str) -> anyhow::Result<()> {
        eprintln!("{msg}");
        Ok(())
    }
}
