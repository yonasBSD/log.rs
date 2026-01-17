// examples/modern-logger.rs
//! Modern Logger Example
//!
//! Demonstrates the ModernLogger with beautiful unicode symbols and modern
//! CLI aesthetics. Inspired by tools like cliclack and ink, this formatter
//! creates polished, professional-looking terminal output.
//!
//! Run with:
//!   cargo run --example modern-logger
//!   cargo run --example modern-logger -- -v      # verbose mode
//!   cargo run --example modern-logger -- --json  # JSON output

use log_rs::{
    banner::{BannerConfig, print as print_banner},
    logging::{LogFormat, ModernBackend, ModernLogger, Printer, Verbosity, log::*, set_logger},
};
use std::thread;
use std::time::Duration;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    let verbosity = if args.contains(&"-q".to_string()) {
        Verbosity::Quiet
    } else if args.contains(&"-vv".to_string()) {
        Verbosity::Trace
    } else if args.contains(&"-v".to_string()) {
        Verbosity::Verbose
    } else {
        Verbosity::Normal
    };

    let format = if args.contains(&"--json".to_string()) {
        LogFormat::Json
    } else {
        LogFormat::Text
    };

    // Initialize the logger with ModernLogger formatter
    let logger = Printer::new(ModernLogger, ModernBackend, format, verbosity);
    set_logger(logger);

    // Print application banner (skip in JSON mode)
    if format == LogFormat::Text {
        let banner = BannerConfig {
            name: "ModernApp",
            version: env!("CARGO_PKG_VERSION"),
            tagline: Some("✨ Showcasing beautiful terminal output"),
            addr: Some("0.0.0.0:3000"),
        };
        print_banner(&banner);

        println!("\n🎨 Running with ModernLogger");
        println!("📊 Verbosity: {:?}", verbosity);
        println!("📝 Format: {:?}\n", format);
    }

    // Demonstrate modern startup sequence
    intro("Bootstrapping application");
    simulate_work(300);

    step("Loading environment configuration");
    simulate_work(200);
    dim("Found .env file with 12 variables");

    step("Initializing logging subsystem");
    simulate_work(150);
    dim("Log level: INFO, Format: JSON");

    step("Connecting to services");
    simulate_work(250);
    ok("Connected to PostgreSQL");
    ok("Connected to Redis");
    ok("Connected to S3");

    outro("Bootstrap complete");

    // Demonstrate deployment workflow
    intro("Deploying application");

    step("Building production assets");
    simulate_work(800);
    dim("Compiled 45 TypeScript files");
    dim("Minified JavaScript: 2.3MB → 780KB");
    dim("Optimized images: 156 files");

    step("Running tests");
    simulate_work(600);
    ok("Unit tests: 234 passed");
    ok("Integration tests: 45 passed");
    warn("Skipped 3 slow tests in CI mode");

    step("Uploading to CDN");
    simulate_work(500);
    dim("Uploaded 203 files to cdn.example.com");
    ok("Cache invalidated successfully");

    step("Deploying to production");
    simulate_work(700);
    ok("Deployed to us-east-1");
    ok("Deployed to eu-west-1");
    ok("Deployed to ap-southeast-1");

    done();

    // Demonstrate error recovery workflow
    intro("Processing batch operations");

    step("Importing user data");
    simulate_work(400);
    ok("Imported 1,250 users");

    step("Validating email addresses");
    simulate_work(350);
    warn("Found 23 invalid email addresses");
    dim("Invalid emails written to errors.log");

    step("Sending welcome emails");
    simulate_work(500);
    err("SMTP server connection failed");
    warn("Retrying in 5 seconds...");
    simulate_work(300);
    ok("Reconnected to SMTP server");
    ok("Sent 1,227 welcome emails");

    outro("Batch processing complete");

    // Demonstrate monitoring/metrics output
    intro("Health check");

    info("System metrics:");
    dim("CPU usage: 23%");
    dim("Memory: 1.2GB / 4GB (30%)");
    dim("Disk: 45GB / 100GB (45%)");

    info("Service status:");
    ok("API server: healthy");
    ok("Database: healthy");
    ok("Cache: healthy");
    warn("Queue: degraded (high latency)");

    outro("Health check complete");

    // Debug and trace examples (only visible in verbose modes)
    debug("Request processing time: 145ms");
    debug("Cache hit rate: 87.3%");
    trace("SQL query: SELECT * FROM users WHERE id = $1");
    trace("Query execution time: 12ms");

    // Final summary
    ok("All systems operational");
    info("Server ready to accept requests");

    if format == LogFormat::Text {
        println!("\n{}", "─".repeat(60));
        println!("💡 Try these commands:");
        println!("  cargo run --example modern-logger           # normal output");
        println!("  cargo run --example modern-logger -- -v     # verbose output");
        println!("  cargo run --example modern-logger -- -vv    # trace output");
        println!("  cargo run --example modern-logger -- --json # JSON output");
        println!("  cargo run --example modern-logger -- -q     # quiet output");
        println!("{}", "─".repeat(60));
    }
}

fn simulate_work(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}
