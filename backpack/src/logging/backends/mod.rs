mod modern;
mod simple;

pub use modern::*;
pub use simple::*;

/// A backend that knows how to *render* formatted strings.
pub trait RenderBackend {
    fn render_error(&self, msg: &str) -> anyhow::Result<()>;
    fn render_info(&self, msg: &str) -> anyhow::Result<()>;
    fn render_remark(&self, msg: &str) -> anyhow::Result<()>;
    fn render_step(&self, msg: &str) -> anyhow::Result<()>;
    fn render_success(&self, msg: &str) -> anyhow::Result<()>;
    fn render_warning(&self, msg: &str) -> anyhow::Result<()>;
    fn render_intro(&self, msg: &str) -> anyhow::Result<()>;
    fn render_outro(&self, msg: &str) -> anyhow::Result<()>;
    fn render_debug(&self, msg: &str) -> anyhow::Result<()>;
    fn render_trace(&self, msg: &str) -> anyhow::Result<()>;
}
