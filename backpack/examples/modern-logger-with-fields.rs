//! Modern Logger Example (with structured fields)

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
    let logger = Printer::new(ModernLogger, ModernBackend::new(), format, verbosity);
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
    ok("Connected to PostgreSQL")
        .field("host", "localhost")
        .field("port", 5432);
    ok("Connected to Redis")
        .field("host", "localhost")
        .field("port", 6379);
    ok("Connected to S3").field("bucket", "modern-app-assets");

    outro("Bootstrap complete");

    // Demonstrate deployment workflow
    intro("Deploying application");

    step("Building production assets");
    simulate_work(800);
    dim("Compiled 45 TypeScript files").field("count", 45);
    dim("Minified JavaScript: 2.3MB → 780KB")
        .field("before_mb", 2.3)
        .field("after_mb", 0.78);
    dim("Optimized images: 156 files").field("count", 156);

    step("Running tests");
    simulate_work(600);
    ok("Unit tests passed").field("count", 234);
    ok("Integration tests passed").field("count", 45);
    warn("Skipped slow tests")
        .field("count", 3)
        .field("reason", "CI mode");

    step("Uploading to CDN");
    simulate_work(500);
    dim("Uploaded files")
        .field("count", 203)
        .field("cdn", "cdn.example.com");
    ok("Cache invalidated successfully");

    step("Deploying to production");
    simulate_work(700);
    ok("Region deployed").field("region", "us-east-1");
    ok("Region deployed").field("region", "eu-west-1");
    ok("Region deployed").field("region", "ap-southeast-1");

    done();

    // Demonstrate error recovery workflow
    intro("Processing batch operations");

    step("Importing user data");
    simulate_work(400);
    ok("Imported users").field("count", 1250);

    step("Validating email addresses");
    simulate_work(350);
    warn("Invalid email addresses found").field("count", 23);
    dim("Invalid emails written to errors.log");

    step("Sending welcome emails");
    simulate_work(500);
    err("SMTP server connection failed").field("server", "smtp.example.com");
    warn("Retrying").field("delay_seconds", 5);
    simulate_work(300);
    ok("Reconnected to SMTP server");
    ok("Sent welcome emails").field("count", 1227);

    outro("Batch processing complete");

    // Demonstrate monitoring/metrics output
    intro("Health check");

    info("System metrics");
    dim("CPU usage").field("percent", 23);
    dim("Memory usage")
        .field("used_gb", 1.2)
        .field("total_gb", 4.0);
    dim("Disk usage")
        .field("used_gb", 45)
        .field("total_gb", 100);

    info("Service status");
    ok("API server healthy");
    ok("Database healthy");
    ok("Cache healthy");
    warn("Queue degraded").field("latency_ms", 1200);

    outro("Health check complete");

    // Debug and trace examples (only visible in verbose modes)
    debug("Request processing time").field("ms", 145);
    debug("Cache hit rate").field("percent", 87.3);
    trace("SQL query").field("query", "SELECT * FROM users WHERE id = $1");
    trace("Query execution time").field("ms", 12);

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
