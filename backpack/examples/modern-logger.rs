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
    logging::{
        LogFormat, ModernBackend, ModernLogger, Printer, Progress, Verbosity, log::*, set_logger,
    },
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

    // Demonstrate deployment workflow with progress tracking
    intro("Deploying application");

    step("Building production assets");
    {
        let mut progress = Progress::with_total("Compiling TypeScript", 45);
        for _ in 1..=45 {
            simulate_work(15);
            progress.tick();
        }
        progress.finish("TypeScript compilation complete");
    }
    dim("Compiled 45 TypeScript files");

    {
        let mut progress = Progress::with_total("Optimizing assets", 100);
        progress.update(30, 100);
        simulate_work(200);
        dim("Minified JavaScript: 2.3MB → 780KB");

        progress.update(65, 100);
        simulate_work(200);
        dim("Optimized images: 156 files");

        progress.update(100, 100);
        simulate_work(100);
        progress.finish("Asset optimization complete");
    }

    step("Running tests");
    {
        let mut progress = Progress::with_total("Running test suites", 279);

        // Unit tests
        for _ in 1..=234 {
            simulate_work(2);
            progress.tick();
        }
        ok("Unit tests: 234 passed");

        // Integration tests
        for _ in 1..=45 {
            simulate_work(8);
            progress.tick();
        }
        ok("Integration tests: 45 passed");

        progress.finish("All tests complete");
    }
    warn("Skipped 3 slow tests in CI mode");

    step("Uploading to CDN");
    {
        let mut progress = Progress::with_total("Uploading files", 203);
        for _ in 1..=203 {
            simulate_work(2);
            progress.tick();
        }
        progress.finish("Upload complete");
    }
    dim("Uploaded 203 files to cdn.example.com");
    ok("Cache invalidated successfully");

    step("Deploying to production");
    {
        let mut progress = Progress::with_total("Rolling out to regions", 3);

        simulate_work(200);
        progress.tick();
        ok("Deployed to us-east-1");

        simulate_work(250);
        progress.tick();
        ok("Deployed to eu-west-1");

        simulate_work(250);
        progress.tick();
        ok("Deployed to ap-southeast-1");

        progress.finish("Deployment complete");
    }

    done();

    // Demonstrate error recovery workflow with progress
    intro("Processing batch operations");

    step("Importing user data");
    {
        let mut progress = Progress::with_total("Reading CSV file", 1250);
        for i in 1..=1250 {
            if i % 100 == 0 {
                simulate_work(30);
            }
            progress.tick();
        }
        progress.finish("Import complete");
    }
    ok("Imported 1,250 users");

    step("Validating email addresses");
    {
        let mut progress = Progress::with_total("Validating emails", 1250);
        for i in 1..=1250 {
            if i % 100 == 0 {
                simulate_work(25);
            }
            progress.tick();
        }
        progress.finish("Validation complete");
    }
    warn("Found 23 invalid email addresses");
    dim("Invalid emails written to errors.log");

    step("Sending welcome emails");
    {
        let mut progress = Progress::with_total("Sending emails", 1227);

        // Simulate SMTP failure partway through
        for i in 1..=500 {
            if i % 50 == 0 {
                simulate_work(20);
            }
            progress.tick();
        }

        err("SMTP server connection failed");
        warn("Retrying in 5 seconds...");
        simulate_work(300);
        ok("Reconnected to SMTP server");

        // Continue from where we left off
        for i in 501..=1227 {
            if i % 50 == 0 {
                simulate_work(20);
            }
            progress.tick();
        }

        progress.finish("Email delivery complete");
    }
    ok("Sent 1,227 welcome emails");

    outro("Batch processing complete");

    // Demonstrate progress without known total
    intro("Discovering resources");
    {
        let mut progress = Progress::new("Scanning filesystem");

        // Simulate discovering files
        for _ in 1..=15 {
            simulate_work(50);
            progress.tick();
        }

        progress.finish("Scan complete");
    }
    ok("Found 15 configuration files");

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

    // Demonstrate progress with dynamic updates
    intro("Analyzing performance");
    {
        let mut progress = Progress::new("Collecting metrics");

        // Start without knowing total
        for _ in 1..=5 {
            simulate_work(80);
            progress.tick();
        }

        // Now we know the total
        progress.update(5, 20);

        for _ in 6..=20 {
            simulate_work(50);
            progress.tick();
        }

        progress.finish("Analysis complete");
    }
    ok("Generated performance report");

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
