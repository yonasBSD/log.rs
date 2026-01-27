use crate::{LogFormat, Verbosity, logging::*};
use std::{sync::Mutex, time::Instant};
use tracing::{Level, debug, error, info, span, span::Span, trace, warn};

pub mod json;
pub mod task_tree;

/// A span that tracks when it was entered so we can compute
/// how long the task took when `outro()` / `done()` is called.
#[derive(Debug)]
pub struct TimedSpan {
    pub span: Span,
    pub start: Instant,
    pub label: String,
}

/// A screen logger that prints formatted messages and, in verbose/trace mode,
/// also emits structured tracing spans.
pub struct Printer<L: FormatLogger, B: RenderBackend> {
    pub inner: L,
    pub backend: B,
    pub tasks: Mutex<Vec<TimedSpan>>,
    pub steps: Mutex<Vec<Span>>,
    pub format: LogFormat,
    pub verbosity: Verbosity,
    pub timestamp: Mutex<TimestampMode>,
}

impl<L: FormatLogger, B: RenderBackend> Printer<L, B> {
    pub fn new(inner: L, backend: B, format: LogFormat, verbosity: Verbosity) -> Self {
        match verbosity {
            Verbosity::Quiet => {
                crate::config::setquiet(true);
                crate::config::setverbose(false);
            }
            Verbosity::Normal => {
                crate::config::setquiet(false);
                crate::config::setverbose(false);
            }
            Verbosity::Verbose | Verbosity::Trace => {
                crate::config::setquiet(false);
                crate::config::setverbose(true);
            }
        }

        let _ = crate::logging::init();

        let printer = Self {
            inner,
            backend,
            tasks: Mutex::new(Vec::new()),
            steps: Mutex::new(Vec::new()),
            format,
            verbosity,
            timestamp: Mutex::new(TimestampMode::Real),
        };

        // Test-only override for deterministic snapshots
        #[cfg(test)]
        {
            *printer.timestamp.lock().unwrap() = TimestampMode::Disabled;
        }

        printer
    }
}

impl<L: FormatLogger, B: RenderBackend> ScreenLogger for Printer<L, B> {
    fn intro(&self, m: &str) {
        if let Some(s) = self.inner.intro(m) {
            match self.format {
                LogFormat::Json => {
                    self.emit_json(LogLevel::Info, &s);
                }
                LogFormat::Text => {
                    let _ = self.backend.render_intro(&s);
                    if self.inner.is_verbose() {
                        info!("{s}");
                    }
                }
            }
        }

        let sp = span!(Level::INFO, "task", message = %m);
        self.tasks.lock().unwrap().push(TimedSpan {
            span: sp,
            start: Instant::now(),
            label: m.to_string(),
        });
    }

    fn outro(&self, m: &str) {
        if let Some(s) = self.inner.outro(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Info, &s),
                LogFormat::Text => {
                    self.steps.lock().unwrap().clear();

                    let task = self.tasks.lock().unwrap().pop();

                    let msg = {
                        #[cfg(not(test))]
                        {
                            if let Some(TimedSpan { span, start, .. }) = task {
                                drop(span);

                                let elapsed = start.elapsed();
                                let timing = format_duration(elapsed);

                                if elapsed.as_millis() > 0 {
                                    format!("{s} (took {timing})")
                                } else {
                                    s
                                }
                            } else {
                                s
                            }
                        }

                        #[cfg(test)]
                        if let Some(TimedSpan { .. }) = task {
                            format!("{s} (took 10ms)")
                        } else {
                            s
                        }
                    };

                    let _ = self.backend.render_outro(&msg);

                    if self.inner.is_verbose() {
                        info!("{msg}");
                    }
                }
            }
        }
    }

    fn done(&self) {
        if let Some(s) = self.inner.done() {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Info, &s),
                LogFormat::Text => {
                    self.steps.lock().unwrap().clear();

                    let task = self.tasks.lock().unwrap().pop();
                    let msg = {
                        #[cfg(not(test))]
                        {
                            if let Some(TimedSpan { span, start, .. }) = task {
                                drop(span);

                                let elapsed = start.elapsed();
                                let timing = format_duration(elapsed);

                                if elapsed.as_millis() > 0 {
                                    format!("{s} (took {timing})")
                                } else {
                                    s
                                }
                            } else {
                                s
                            }
                        }

                        #[cfg(test)]
                        if let Some(TimedSpan { .. }) = task {
                            format!("{s} (took 10ms)")
                        } else {
                            s
                        }
                    };

                    let _ = self.backend.render_outro(&msg);

                    if self.inner.is_verbose() {
                        info!("{msg}");
                    }
                }
            }
        }
    }

    fn step(&self, m: &str) {
        if let Some(s) = self.inner.step(m) {
            match self.format {
                LogFormat::Json => {
                    self.emit_json(LogLevel::Info, &s);
                }
                LogFormat::Text => {
                    let _ = self.backend.render_step(&s);

                    if self.inner.is_verbose() {
                        let sp = span!(Level::INFO, "step", message = %m);
                        self.steps.lock().unwrap().push(sp);
                        info!("{s}");
                    }
                }
            }
        }
    }

    fn ok(&self, m: &str) {
        if let Some(s) = self.inner.ok(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Info, &s),
                LogFormat::Text => {
                    let _ = self.backend.render_success(&s);
                }
            }
        }
    }

    fn warn(&self, m: &str) {
        if let Some(s) = self.inner.warn(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Warn, &s),
                LogFormat::Text => {
                    let _ = self.backend.render_warning(&s);
                    warn!("{s}");
                }
            }
        }
    }

    fn err(&self, m: &str) {
        let s = self.inner.err(m);

        match self.format {
            LogFormat::Json => self.emit_json(LogLevel::Error, &s),
            LogFormat::Text => {
                let _ = self.backend.render_error(&s);
                error!("{s}");
            }
        }
    }

    fn info(&self, m: &str) {
        if let Some(s) = self.inner.info(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Info, &s),
                LogFormat::Text => {
                    let _ = self.backend.render_info(&s);
                }
            }
        }
    }

    fn dim(&self, m: &str) {
        if let Some(s) = self.inner.dim(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Debug, &s),
                LogFormat::Text => {
                    let _ = self.backend.render_remark(&s);
                }
            }
        }
    }

    fn debug(&self, m: &str) {
        if let Some(s) = self.inner.debug(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Debug, &s),
                LogFormat::Text => {
                    debug!("{s}");
                }
            }
        }
    }

    fn trace(&self, m: &str) {
        if let Some(s) = self.inner.trace(m) {
            match self.format {
                LogFormat::Json => self.emit_json(LogLevel::Trace, &s),
                LogFormat::Text => {
                    trace!("{s}");
                }
            }
        }
    }

    fn dump_tree(&self) {
        self.dump_task_tree();
    }

    fn progress(&self, label: &str, current: u64, total: Option<u64>, finished: bool) {
        match self.format {
            LogFormat::Json => {
                // Emit a structured progress event
                let mut fields = Fields::new();
                fields.insert("label".into(), label.to_string());
                fields.insert("current".into(), current.to_string());
                if let Some(t) = total {
                    fields.insert("total".into(), t.to_string());
                }
                fields.insert("finished".into(), finished.to_string());

                // Use the Progress level you already added
                self.emit_json(LogLevel::Progress, label);
            }
            LogFormat::Text => {
                let _ = self
                    .backend
                    .render_progress(label, current, total, finished);
            }
        }
    }
}

impl<L, B> GlobalLoggerType for Printer<L, B>
where
    L: FormatLogger + Send + Sync + 'static,
    B: RenderBackend + Send + Sync + 'static,
    Self: EmitsEvents,
{
}
