# Changelog

All notable changes to this project will be documented in this file.

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