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

use your_crate::{
    logger::{set_logger, log, Printer, ModernLogger, Verbosity, LogFormat},
    banner::{BannerConfig, print as print_banner},
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
    let logger = Printer::new(ModernLogger, format);
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
    log().intro("Bootstrapping application");
    simulate_work(300);

    log().step("Loading environment configuration");
    simulate_work(200);
    log().dim("Found .env file with 12 variables");

    log().step("Initializing logging subsystem");
    simulate_work(150);
    log().dim("Log level: INFO, Format: JSON");

    log().step("Connecting to services");
    simulate_work(250);
    log().ok("Connected to PostgreSQL");
    log().ok("Connected to Redis");
    log().ok("Connected to S3");

    log().outro("Bootstrap complete");

    // Demonstrate deployment workflow
    log().intro("Deploying application");

    log().step("Building production assets");
    simulate_work(800);
    log().dim("Compiled 45 TypeScript files");
    log().dim("Minified JavaScript: 2.3MB → 780KB");
    log().dim("Optimized images: 156 files");

    log().step("Running tests");
    simulate_work(600);
    log().ok("Unit tests: 234 passed");
    log().ok("Integration tests: 45 passed");
    log().warn("Skipped 3 slow tests in CI mode");

    log().step("Uploading to CDN");
    simulate_work(500);
    log().dim("Uploaded 203 files to cdn.example.com");
    log().ok("Cache invalidated successfully");

    log().step("Deploying to production");
    simulate_work(700);
    log().ok("Deployed to us-east-1");
    log().ok("Deployed to eu-west-1");
    log().ok("Deployed to ap-southeast-1");

    log().done("Deployment successful");

    // Demonstrate error recovery workflow
    log().intro("Processing batch operations");

    log().step("Importing user data");
    simulate_work(400);
    log().ok("Imported 1,250 users");

    log().step("Validating email addresses");
    simulate_work(350);
    log().warn("Found 23 invalid email addresses");
    log().dim("Invalid emails written to errors.log");

    log().step("Sending welcome emails");
    simulate_work(500);
    log().err("SMTP server connection failed");
    log().warn("Retrying in 5 seconds...");
    simulate_work(300);
    log().ok("Reconnected to SMTP server");
    log().ok("Sent 1,227 welcome emails");

    log().outro("Batch processing complete");

    // Demonstrate monitoring/metrics output
    log().intro("Health check");

    log().info("System metrics:");
    log().dim("CPU usage: 23%");
    log().dim("Memory: 1.2GB / 4GB (30%)");
    log().dim("Disk: 45GB / 100GB (45%)");

    log().info("Service status:");
    log().ok("API server: healthy");
    log().ok("Database: healthy");
    log().ok("Cache: healthy");
    log().warn("Queue: degraded (high latency)");

    log().outro("Health check complete");

    // Debug and trace examples (only visible in verbose modes)
    log().debug("Request processing time: 145ms");
    log().debug("Cache hit rate: 87.3%");
    log().trace("SQL query: SELECT * FROM users WHERE id = $1");
    log().trace("Query execution time: 12ms");

    // Final summary
    log().ok("All systems operational");
    log().info("Server ready to accept requests");

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
