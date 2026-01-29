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

use log_rs::{
    banner::{BannerConfig, print as print_banner},
    logging::{
        LogFormat, Printer, Progress, SimpleBackend, SimpleLogger, Verbosity, log::*, set_logger,
    },
};
use std::thread;
use std::time::Duration;

fn main() {
    // Parse command line arguments for verbosity
    let args: Vec<String> = std::env::args().collect();

    let verbosity: Verbosity = if args.contains(&"-q".to_string()) {
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

    // Initialize the logger with SimpleLogger formatter
    let logger = Printer::new(SimpleLogger, SimpleBackend, format, verbosity);
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
    info("Starting application...");
    ok("Configuration loaded successfully");
    dim("Using default settings");

    // Demonstrate a task with intro/outro
    intro("Initializing database");
    simulate_work(500);
    step("Connecting to database");
    simulate_work(300);
    step("Running migrations");
    simulate_work(400);
    step("Seeding initial data");
    simulate_work(200);
    outro("Database initialized");

    // Demonstrate warnings and errors
    warn("Cache is not configured - performance may be degraded");

    // Demonstrate progress tracking with known total
    intro("Downloading dependencies");
    {
        let mut progress = Progress::with_total("Fetching packages", 5);
        for _ in 1..=5 {
            simulate_work(200);
            progress.tick();
        }
        progress.finish("All packages downloaded");
    }

    // Demonstrate progress tracking without known total
    intro("Processing files");
    {
        let mut progress = Progress::new("Scanning directory");
        for _ in 1..=8 {
            simulate_work(100);
            progress.tick();
        }
        progress.finish("Scan complete");
    }

    // Demonstrate progress with manual updates
    intro("Building application");
    {
        let mut progress = Progress::with_total("Compiling", 100);

        // Simulate compilation progress
        progress.update(10, 100);
        simulate_work(150);

        progress.update(35, 100);
        simulate_work(200);

        progress.update(60, 100);
        simulate_work(180);

        progress.update(85, 100);
        simulate_work(120);

        progress.update(100, 100);
        simulate_work(100);

        progress.finish("Build complete");
    }

    // Demonstrate another task
    intro("Starting HTTP server");
    step("Binding to port 8080");
    simulate_work(100);
    step("Registering routes");
    simulate_work(150);
    step("Starting worker threads");
    simulate_work(200);
    done();

    // Debug/trace messages (only visible in verbose mode)
    debug("Debug: Connection pool size: 10");
    trace("Trace: Request headers: {\"user-agent\": \"example/1.0\"}");

    // Demonstrate error handling with progress
    intro("Processing background jobs");
    {
        let mut progress = Progress::with_total("Running jobs", 3);

        step("Job 1: Send email notifications");
        simulate_work(300);
        progress.tick();
        ok("Sent 150 notifications");

        step("Job 2: Generate reports");
        simulate_work(400);
        progress.tick();
        err("Failed to generate report: database timeout");

        step("Job 3: Clean up temp files");
        simulate_work(200);
        progress.tick();
        ok("Deleted 45 temporary files");

        progress.finish("Jobs completed (with errors)");
    }

    // Demonstrate progress with dynamic total changes
    intro("Analyzing data");
    {
        let mut progress = Progress::new("Discovering files");

        // First pass - counting
        for _ in 1..=3 {
            simulate_work(80);
            progress.tick();
        }

        // Now we know the total
        progress.update(3, 10);

        // Continue processing with known total
        for _ in 4..=10 {
            simulate_work(100);
            progress.tick();
        }

        progress.finish("Analysis complete");
    }

    // Final status
    ok("Application running successfully");
    info("Press Ctrl+C to stop");

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
