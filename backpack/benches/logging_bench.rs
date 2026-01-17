use backpack::logging::{LogFormat, Printer, SimpleBackend, SimpleLogger, Verbosity};
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_text_mode_no_fields(c: &mut Criterion) {
    let printer = Printer::new(
        SimpleLogger,
        SimpleBackend,
        LogFormat::Text,
        Verbosity::Normal,
    );

    c.bench_function("text_mode_info_no_fields", |b| {
        b.iter(|| {
            printer.info("Hello world");
        });
    });
}

fn bench_text_mode_with_fields(c: &mut Criterion) {
    let printer = Printer::new(
        SimpleLogger,
        SimpleBackend,
        LogFormat::Text,
        Verbosity::Normal,
    );

    c.bench_function("text_mode_info_with_fields", |b| {
        b.iter(|| {
            printer
                .info("Hello world")
                .field("user_id", 42)
                .field("role", "admin");
        });
    });
}

fn bench_json_mode_with_fields(c: &mut Criterion) {
    let printer = Printer::new(
        SimpleLogger,
        SimpleBackend,
        LogFormat::Json,
        Verbosity::Normal,
    );

    c.bench_function("json_mode_info_with_fields", |b| {
        b.iter(|| {
            printer
                .info("Hello world")
                .field("user_id", 42)
                .field("role", "admin");
        });
    });
}

criterion_group!(
    benches,
    bench_text_mode_no_fields,
    bench_text_mode_with_fields,
    bench_json_mode_with_fields
);
criterion_main!(benches);
