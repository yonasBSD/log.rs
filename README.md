# 🎨 log.rs

> Beautiful, ergonomic logging and banners for Rust CLI applications

![Licenses](https://github.com/yonasBSD/log.rs/actions/workflows/licenses.yaml/badge.svg)
![Linting](https://github.com/yonasBSD/log.rs/actions/workflows/lint.yaml/badge.svg)
![Testing](https://github.com/yonasBSD/log.rs/actions/workflows/test-with-coverage.yaml/badge.svg)
![Packaging](https://github.com/yonasBSD/log.rs/actions/workflows/release-packaging.yaml/badge.svg)
![Cross-Build](https://github.com/yonasBSD/log.rs/actions/workflows/cross-build.yaml/badge.svg)

![Security Audit](https://github.com/yonasBSD/log.rs/actions/workflows/security.yaml/badge.svg)
![Scorecard Audit](https://github.com/yonasBSD/log.rs/actions/workflows/scorecard.yaml/badge.svg)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_log.rs&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=yonasBSD_log.rs)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_log.rs&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=yonasBSD_log.rs)
[![Vulnerabilities](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_log.rs&metric=vulnerabilities)](https://sonarcloud.io/summary/new_code?id=yonasBSD_log.rs)
<!--[![codecov](https://codecov.io/gh/yonasBSD/log.rs/branch/main/graph/badge.svg?token=SLIHSUWHT2)](https://codecov.io/gh/yonasBSD/log.rs)-->
<!--[![ghcr.io](https://img.shields.io/badge/ghcr.io-download-blue)](https://github.com/yonasBSD/log.rs/pkgs/container/log.rs)-->
<!--[![Docker Pulls](https://img.shields.io/docker/pulls/log.rs/example.svg)](https://hub.docker.com/r/log.rs/example)-->
<!--[![Quay.io](https://img.shields.io/badge/Quay.io-download-blue)](https://quay.io/repository/log.rs/example)-->

![GitHub last commit](https://img.shields.io/github/last-commit/yonasBSD/log.rs)
[![Dependency Status](https://deps.rs/repo/github/yonasBSD/log.rs/status.svg)](https://deps.rs/repo/github/yonasBSD/log.rs)
![Rust](https://img.shields.io/badge/Built%20With-Rust-orange?logo=rust)
[![GitHub Release](https://img.shields.io/github/release/yonasBSD/log.rs.svg)](https://github.com/yonasBSD/log.rs/releases/latest)
[![License](https://img.shields.io/github/license/yonasBSD/log.rs.svg)](https://github.com/yonasBSD/log.rs/blob/main/LICENSE.txt)
<!--[![Matrix Chat](https://img.shields.io/matrix/vaultwarden:matrix.org.svg?logo=matrix)](https://matrix.to/#/#vaultwarden:matrix.org)-->

---

## ✨ Features

**🪵 Production-Ready Logging**
- Cargo-style verbosity levels (Quiet → Normal → Verbose → Trace)
- Dual output modes: beautiful text or structured JSON
- Automatic task timing and span management
- Global singleton for ergonomic API
- Zero-cost quiet mode suppression

**🎯 Elegant Banners**
- Clean ASCII art for professional first impressions
- Smart address formatting (wildcard binds show as `:PORT`)
- Optional taglines and version display
- ANSI color support with graceful fallbacks

**🎭 Multiple Styles**
- `SimpleLogger`: Classic ASCII symbols for maximum compatibility
- `ModernLogger`: Beautiful unicode for modern terminals
- Extensible formatter trait for custom styles

---

## 🚀 Quick Start

```bash
cargo add log-rs
```

### Basic Logging

```rust
use log_rs::{
    logging::{set_logger, log, Printer, ModernLogger, Verbosity, LogFormat},
    banner::{BannerConfig, print as print_banner},
};

fn main() {
    // Initialize logger once at startup
    let logger = Printer::new(ModernLogger, LogFormat::Text);
    set_logger(logger);

    // Use anywhere in your app
    log().intro("Deploying application");
    log().step("Building assets");
    log().ok("Build successful");
    log().outro("Deployment complete");
    // → Deploying application
    // ⠿ Building assets
    // ✔ Build successful
    // ✔ Deployment complete (took 2.3s)
}
```

### Beautiful Banners

```rust
let banner = BannerConfig {
    name: "MyAPI",
    version: "1.0.0",
    tagline: Some("Fast and reliable REST API"),
    addr: Some("0.0.0.0:8080"),
};

print_banner(&banner);
```

**Output:**
```
   ____    __
  / __/___/ /  ___
 / _// __/ _ \/ _ \
/___/\__/_//_/\___/ v1.0.0
Fast and reliable REST API
 ⇨ MyAPI listening on :8080
```

---

## 📖 Documentation

### Verbosity Levels

Control output detail with four levels:

| Level | Flag | Usage | Output |
|-------|------|-------|--------|
| **Quiet** | `-q` | Cron jobs, CI | Errors only |
| **Normal** | _(default)_ | Standard CLI | Success, warnings, info |
| **Verbose** | `-v` | Troubleshooting | + Debug logs, tracing spans |
| **Trace** | `-vv` | Deep debugging | + Trace logs, full diagnostics |

### Output Formats

**Text Mode** (Human-Friendly)
```text
✔ Server started
⠿ Processing request
⚠ Cache miss for key: user_123
✗ Database connection failed
```

**JSON Mode** (Machine-Friendly)
```json
{"level":"info","message":"✔ Server started","timestamp":"2026-01-15T10:30:00Z"}
{"level":"warn","message":"⚠ Cache miss","timestamp":"2026-01-15T10:30:01Z"}
{"level":"error","message":"✗ Database connection failed","timestamp":"2026-01-15T10:30:02Z"}
```

### Logger API

```rust
// Status messages
log().ok("Operation successful");
log().warn("Potential issue detected");
log().err("Operation failed");
log().info("Informational message");
log().dim("Muted remark");

// Task management
log().intro("Starting deployment");  // Begins a timed task
log().step("Building assets");       // Progress indicator
log().outro("Deployment complete");  // Ends task, shows duration

// Debug output (verbose mode only)
log().debug("Cache hit rate: 87%");
log().trace("SQL: SELECT * FROM users");
```

### Banner Configuration

```rust
pub struct BannerConfig<'a> {
    pub name: &'a str,              // Required: app name
    pub version: &'a str,           // Required: version string
    pub tagline: Option<&'a str>,   // Optional: description
    pub addr: Option<&'a str>,      // Optional: bind address
}
```

**Address Formatting:**
- `127.0.0.1:8080` → displays as `127.0.0.1:8080`
- `0.0.0.0:8080` → displays as `:8080` (cleaner for wildcards)
- `[::]:8080` → displays as `:8080`
- Invalid/empty → omitted from banner

---

## 🎨 Formatter Comparison

### SimpleLogger (ASCII)
```
+ Configuration loaded
! Cache not configured
X Database timeout
* Processing items
→ Starting deployment
✓ Deployment complete
```

### ModernLogger (Unicode)
```
✔ Configuration loaded
⚠ Cache not configured
✗ Database timeout
⠿ Processing items
→ Starting deployment
✔ Deployment complete
🔍 Debug information
… Trace details
```

---

## 📚 Examples

Run the included examples to see the loggers in action:

```bash
# Simple ASCII logger
cargo run --example simple-logger
cargo run --example simple-logger -- -v

# Modern unicode logger
cargo run --example modern-logger
cargo run --example modern-logger -- --json

# Quiet mode (errors only)
cargo run --example modern-logger -- -q
```

---

## 🏗️ Architecture

**Two-Layer Design:**

1. **FormatLogger** → Formats messages into styled strings
   - `SimpleLogger`: ASCII symbols
   - `ModernLogger`: Unicode symbols
   - Implement your own for custom styles

2. **ScreenLogger** → Prints formatted messages
   - `Printer`: Manages spans, timing, output routing
   - Integrates with `tracing` for structured logs

**Benefits:**
- Clean separation of formatting and I/O
- Easy to test formatters without side effects
- Trivial to add new output styles
- Swap backends without changing user code

---

## 🤝 Integration

### With Tracing

The logger integrates seamlessly with the `tracing` ecosystem:

```rust
use tracing::info;

// In verbose mode, log() calls emit tracing events
log().intro("Processing batch");  // Creates a tracing span
log().step("Item 1");              // Nested span
log().outro("Batch complete");    // Closes span with timing

// Regular tracing works alongside
info!("Direct tracing event");
```

### With Clap

```rust
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    quiet: bool,
    
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let cli = Cli::parse();
    
    let verbosity = match (cli.quiet, cli.verbose) {
        (true, _) => Verbosity::Quiet,
        (_, 0) => Verbosity::Normal,
        (_, 1) => Verbosity::Verbose,
        (_, _) => Verbosity::Trace,
    };
    
    let logger = Printer::new(ModernLogger, LogFormat::Text);
    set_logger(logger);
    
    // Your app logic...
}
```

---

## 🎯 Design Philosophy

**Great CLIs are invisible until you need them.**

This toolkit prioritizes:

- **Ergonomics** → `log().ok("done")` beats dependency injection
- **Clarity** → Visual symbols communicate status instantly
- **Performance** → Lazy formatting, zero-cost abstractions
- **Flexibility** → Start simple, scale to production
- **Professionalism** → Output that looks polished everywhere

Whether you're building a quick script or a production service, these tools adapt to your needs without getting in your way.

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test logger_tests
cargo test banner_tests

# Run with output
cargo test -- --nocapture
```

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

Inspired by:
- [Cargo's](https://github.com/rust-lang/cargo) excellent CLI output
- [cliclack](https://github.com/natemoo-re/clack) for modern terminal aesthetics
- [Echo](https://echo.labstack.com/) and [Express](https://expressjs.com/) for startup banner design
- The [tracing](https://github.com/tokio-rs/tracing) ecosystem for structured logging

---

## 🤔 FAQ

**Q: Can I use both SimpleLogger and ModernLogger in the same app?**  
A: No, you set one logger globally at startup. Choose based on your target environment.

**Q: Does quiet mode completely silence output?**  
A: No, errors always print. Quiet mode is for automation where you only want failures.

**Q: Can I customize the banner ASCII art?**  
A: Currently no, but you can print your own before calling `print_banner()`. Open an issue if you need this feature!

**Q: Is this production-ready?**  
A: Yes! The logger is built on the battle-tested `tracing` ecosystem and includes comprehensive tests.

**Q: How do I capture logs in tests?**  
A: Use `tracing-subscriber` test utilities or capture stdout/stderr. See the test files for examples.

---

<div align="center">

**[Documentation](https://docs.rs/...)** • **[Crates.io](https://crates.io/crates/...)** • **[Report Bug](https://github.com/yonasBSD/log.rs/issues)** • **[Request Feature](https://github.com/yonasBSD/log.rs/issues)**

Made with ❤️ for the Rust CLI community

</div>
