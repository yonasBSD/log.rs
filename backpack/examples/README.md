# Examples

## Normal output
cargo run --example simple-logger
cargo run --example modern-logger

## Verbose mode (shows debug messages)
cargo run --example simple-logger -- -v
cargo run --example modern-logger -- -v

## Trace mode (shows trace messages)
cargo run --example simple-logger -- -vv
cargo run --example modern-logger -- -vv

## Quiet mode (errors only)
cargo run --example simple-logger -- -q
cargo run --example modern-logger -- -q

## JSON output (modern-logger only shown, but works for both)
cargo run --example modern-logger -- --json

### File Structure
```console
examples/
├── simple-logger.rs
└── modern-logger.rs
```
