# Changelog

All notable changes to this project will be documented in this file.

## [0.2.1] - 2025_010_001_018_009_038_539646000_America_slash_New_York_2025_W040_003_2025_274_1759356578_539646000

### Updated
- **README.md**:
  - Corrected usage examples to show that `format_number` requires both a number (as `&str`) and a group size (`""`, `"4"`, or `usize`).
  - Added demonstrations for:
    - Default group size (`""`)
    - Explicit group size as `&str`
    - Explicit group size as `usize`
    - Decimal numbers
    - Negative numbers

---

## [0.2.0] - 2025_010_001_017_050_016_216531100_America_slash_New_York_2025_W040_003_2025_274_1759355416_216531100

### Added
- Extended `format_number()` to accept both **`&str`** and **`usize`** for the `group_size` argument:
  - `format_number("12345", "")` → `"012_345"`
  - `format_number("12345", "4")` → `"0001_2345"`
  - `format_number("12345", 4)` → `"0001_2345"`
- Introduced **`IntoGroupSize` trait** internally to unify conversion from `&str` and `usize` into a validated group size.
- Added panic safety: `group_size = 0` (either as `"0"` or `0`) will cause a clear panic `"group_size must be positive"`.

### Tests
- Expanded tests to cover:
  - Using `&str` group sizes.
  - Using `usize` group sizes.
  - Panic conditions for `0`.
  - Negative numbers and decimals.

---

## [0.1.0] - 2025_010_001_017_012_045_901194900_America_slash_New_York_2025_W040_003_2025_274_1759353165_901194900

### Added
- Initial release of **`rust-functions`** crate.
- Introduced `number_formatting` module.
- Added `format_number()` function:
  - Supports grouping integers and fractional parts with underscores.
  - Handles optional `group_size` parameter (default = 3).
  - Pads numbers with leading zeros to align with grouping.
  - Special formatting for decimals: `_decimal_point_` marker.
  - Handles negative values by prefixing with `negative_`.

### Example
```rust
use rust_functions::number_formatting::format_number;

fn main() {
    let n = "12345";
    println!("{}", format_number(n, "")); // "012_345"
}
```

---