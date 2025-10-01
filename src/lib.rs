pub mod number_formatting {
    const DEFAULT_DECIMAL_VALUE: usize = 3;

    fn parse_group_size(group_size: &str) -> usize {
        if group_size.is_empty() {
            return DEFAULT_DECIMAL_VALUE;
        }
        let k: usize = group_size.parse().expect("group_size must be a positive integer");
        if k == 0 {
            panic!("group_size must be positive");
        }
        k
    }

    fn group_integer(n: u128, k: usize) -> String {
        let mut s = n.to_string();
        if s == "0" {
            return "0".repeat(k);
        }
        let pad = (k - (s.len() % k)) % k;
        if pad > 0 {
            s = "0".repeat(pad) + &s;
        }
        s.as_bytes()
            .chunks(k)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect::<Vec<_>>()
            .join("_")
    }

    fn group_fractional(frac: &str, k: usize) -> String {
        let mut f = if frac.is_empty() { "0".to_string() } else { frac.to_string() };
        let pad = (k - (f.len() % k)) % k;
        if pad > 0 {
            f.push_str(&"0".repeat(pad));
        }
        f.as_bytes()
            .chunks(k)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect::<Vec<_>>()
            .join("_")
    }

    /// Public function
    pub fn format_number(x: &str, group_size: &str) -> String {
        let k = parse_group_size(group_size);
        let mut s = x.trim().to_string();

        let mut negative = false;
        if s.starts_with('+') || s.starts_with('-') {
            negative = s.starts_with('-');
            s = s[1..].to_string();
        }

        if s.contains('.') {
            let parts: Vec<&str> = s.splitn(2, '.').collect();
            let int_part = if parts[0].is_empty() { "0" } else { parts[0] };
            let frac_part = if parts.len() > 1 { parts[1] } else { "0" };

            let int_val: u128 = int_part.parse().unwrap();
            let int_fmt = group_integer(int_val, k);
            let frac_fmt = group_fractional(frac_part, k);

            let mut out = format!("{}_decimal_point_{}", int_fmt, frac_fmt);
            if negative {
                out = format!("negative_{}", out);
            }
            return out;
        }

        if s.is_empty() {
            s = "0".to_string();
        }
        let int_val: u128 = s.parse().unwrap();
        let mut out = group_integer(int_val, k);
        if negative {
            out = format!("negative_{}", out);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::number_formatting::format_number;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number("12345", ""), "012_345");
        assert_eq!(format_number("12345", "4"), "0001_2345");
        assert_eq!(format_number("1.23456", "4"), "0001_decimal_point_2345_6000");
        assert_eq!(format_number("-1234", ""), "negative_001_234");
    }
}