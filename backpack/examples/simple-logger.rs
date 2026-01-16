// examples/simple-logger.rs
//! Simple Logger Example
//!
//! Demonstrates the SimpleLogger with basic ASCII symbols and no-frills output.
//! Perfect for environments where unicode support is limited or you prefer
//! a more traditional CLI aesthetic.
//!
//! Run with:
//!   cargo run --example simple-logger
//!   cargo run --example simple-logger -- -v     # verbose mode
//!   cargo run --example simple-logger -- -q     # quiet mode

use your_crate::{
    logger::{set_logger, log, Printer, SimpleLogger, Verbosity, LogFormat},
    banner::{BannerConfig, print as print_banner},
};
use std::thread;
use std::time::Duration;

fn main() {
    // Parse command line arguments for verbosity
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

    // Initialize the logger with SimpleLogger formatter
    let logger = Printer::new(SimpleLogger, LogFormat::Text);
    set_logger(logger);

    // Print application banner
    let banner = BannerConfig {
        name: "SimpleApp",
        version: env!("CARGO_PKG_VERSION"),
        tagline: Some("Demonstrating SimpleLogger with ASCII output"),
        addr: Some("127.0.0.1:8080"),
    };
    print_banner(&banner);

    println!("\nRunning with verbosity: {:?}\n", verbosity);

    // Demonstrate different log levels
    log().info("Starting application...");
    log().ok("Configuration loaded successfully");
    log().dim("Using default settings");

    // Demonstrate a task with intro/outro
    log().intro("Initializing database");
    simulate_work(500);
    log().step("Connecting to database");
    simulate_work(300);
    log().step("Running migrations");
    simulate_work(400);
    log().step("Seeding initial data");
    simulate_work(200);
    log().outro("Database initialized");

    // Demonstrate warnings and errors
    log().warn("Cache is not configured - performance may be degraded");

    // Demonstrate another task
    log().intro("Starting HTTP server");
    log().step("Binding to port 8080");
    simulate_work(100);
    log().step("Registering routes");
    simulate_work(150);
    log().step("Starting worker threads");
    simulate_work(200);
    log().done("Server ready");

    // Debug/trace messages (only visible in verbose mode)
    log().debug("Debug: Connection pool size: 10");
    log().trace("Trace: Request headers: {\"user-agent\": \"example/1.0\"}");

    // Demonstrate error handling
    log().intro("Processing background jobs");
    log().step("Job 1: Send email notifications");
    simulate_work(300);
    log().ok("Sent 150 notifications");

    log().step("Job 2: Generate reports");
    simulate_work(400);
    log().err("Failed to generate report: database timeout");

    log().step("Job 3: Clean up temp files");
    simulate_work(200);
    log().ok("Deleted 45 temporary files");
    log().outro("Background jobs completed (with errors)");

    // Final status
    log().ok("Application running successfully");
    log().info("Press Ctrl+C to stop");

    println!("\n{}", "=".repeat(60));
    println!("Try running with different verbosity levels:");
    println!("  cargo run --example simple-logger         # normal");
    println!("  cargo run --example simple-logger -- -v   # verbose");
    println!("  cargo run --example simple-logger -- -vv  # trace");
    println!("  cargo run --example simple-logger -- -q   # quiet");
    println!("{}", "=".repeat(60));
}

fn simulate_work(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}
