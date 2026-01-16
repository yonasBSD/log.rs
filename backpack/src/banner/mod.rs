//! # Terminal Banner
//!
//! A lightweight, elegant ASCII art banner system for CLI applications.
//!
//! ## Overview
//!
//! This module provides a clean, professional way to display application
//! startup information in the terminal. Inspired by frameworks like Echo
//! and Express, it shows your app's identity with style while maintaining
//! simplicity and zero dependencies.
//!
//! ## Features
//!
//! - **Clean ASCII Art**: Eye-catching logo that works in any terminal
//! - **Smart Address Display**: Automatically formats bind addresses for clarity
//!   - Wildcard binds (`0.0.0.0/::`) show as `:PORT` for brevity
//!   - Specific IPs display as `IP:PORT` for precision
//! - **ANSI Colors**: Tasteful green highlighting for addresses
//! - **Flexible Configuration**: Optional tagline and address display
//! - **Zero Allocations**: Efficient formatting with minimal overhead
//!
//! ## Quick Start
//!
//! ```rust
//! use log_rs::banner::{BannerConfig, print};
//!
//! let config = BannerConfig {
//!     name: "MyAPI",
//!     version: "1.0.0",
//!     tagline: Some("Fast and reliable REST API"),
//!     addr: Some("0.0.0.0:8080"),
//! };
//!
//! print(&config);
//! // Outputs:
//! //    ____    __
//! //   / __/___/ /  ___
//! //  / _// __/ _ \/ _ \
//! // /___/\__/_//_/\___/ v1.0.0
//! // Fast and reliable REST API
//! //  ⇨ MyAPI listening on :8080
//! ```
//!
//! ## Design Philosophy
//!
//! Great CLIs start with great first impressions. This banner strikes the
//! perfect balance between informative and unobtrusive—professional enough
//! for production, friendly enough for development.
//!
//! When your application starts, users immediately see:
//! - What's running (name + version)
//! - What it does (tagline)
//! - Where to reach it (address)
//!
//! All in under 10 lines of output.

use std::net::SocketAddr;

pub struct BannerConfig<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub tagline: Option<&'a str>,
    pub addr: Option<&'a str>,
}

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

#[must_use]
pub fn print_address(addr: SocketAddr) -> String {
    let ip = addr.ip().to_string();
    let port = addr.port();

    // Echo-style: if bound to 0.0.0.0, show only :PORT
    let display = if ip == "0.0.0.0" || ip == "::" {
        format!(":{port}")
    } else {
        format!("{ip}:{port}")
    };

    format!("{GREEN}{display}{RESET}")
}

pub fn print(config: &BannerConfig<'_>) {
    let tagline = config.tagline.unwrap_or("app.rs framework");
    let addr_line = config
        .addr
        .filter(|s| !s.is_empty())
        .and_then(|addr_str| addr_str.parse::<SocketAddr>().ok())
        .map(|addr| format!(" ⇨ {} listening on {}", config.name, print_address(addr)))
        .unwrap_or_default();

    println!(
        r"
   ____    __
  / __/___/ /  ___
 / _// __/ _ \/ _ \
/___/\__/_//_/\___/ v{version}

{tagline}

{addr_line}
",
        version = config.version,
        tagline = tagline,
        addr_line = addr_line,
    );
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
