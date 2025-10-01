# rust-functions

A collection of small, reusable Rust utility functions.  
The first release includes `format_number`, a helper for formatting numbers in a human-friendly way.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-functions = "0.1"
```

## Usage

```rust
use rust_functions::format_number;

fn main() {
    let n = 1234567;
    println!("{}", format_number(n)); // "001_234_567"
}
```

## License

MIT