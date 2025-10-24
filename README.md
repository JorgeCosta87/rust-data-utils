# Rust Data Utils

A Rust library for statistical analysis and text processing.

## Modules

### Statistics
- `mean(data: &[i32]) -> f64` - Calculate arithmetic mean
- `median(data: &[i32]) -> i32` - Calculate median value
- `moda(data: &[i32]) -> i32` - Find most frequent value

### Codex
- `convert_to_pig_latin(s: &str) -> String` - Convert text to pig latin
- `safe_convert_to_pig_latin(s: &str) -> String` - UTF-8 safe pig latin conversion

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
rust-data-utils = { path = "../rust-data-utils" }
```

Example:

```rust
use rust_data_utils::*;

fn main() {
    // Statistics
    let data = vec![1, 2, 3, 4, 5];
    println!("Mean: {}", mean(&data));
    println!("Median: {}", median(&data));
    println!("Mode: {}", moda(&data));
    
    // Text processing
    println!("{}", convert_to_pig_latin("Solana"));
    println!("{}", safe_convert_to_pig_latin("안olana"));
}
```

## Running

```bash
cargo run
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_mean

```
