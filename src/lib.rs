pub mod number_formatting {
    const DEFAULT_DECIMAL_VALUE: usize = 3;

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

    fn format_number_internal(x: &str, k: usize) -> String {
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

    /// Trait to accept either `&str` or `usize` as group size
    pub trait IntoGroupSize {
        fn into_group_size(self) -> usize;
    }

    impl IntoGroupSize for &str {
        fn into_group_size(self) -> usize {
            if self.is_empty() {
                return DEFAULT_DECIMAL_VALUE;
            }
            let k: usize = self.parse().expect("group_size must be a positive integer");
            if k == 0 {
                panic!("group_size must be positive");
            }
            k
        }
    }

    impl IntoGroupSize for usize {
        fn into_group_size(self) -> usize {
            if self == 0 {
                panic!("group_size must be positive");
            }
            self
        }
    }

    /// Now one function works for both `&str` and `usize`
    pub fn format_number<T: IntoGroupSize>(x: &str, group_size: T) -> String {
        let k = group_size.into_group_size();
        format_number_internal(x, k)
    }
}

#[cfg(test)]
mod tests {
    use super::number_formatting::format_number;

    #[test]
    fn test_format_number_with_str() {
        assert_eq!(format_number("12345", ""), "012_345"); // default group size = 3
        assert_eq!(format_number("12345", "4"), "0001_2345");
        assert_eq!(format_number("1.23456", "4"), "0001_decimal_point_2345_6000");
        assert_eq!(format_number("-1234", ""), "negative_001_234");
    }

    #[test]
    fn test_format_number_with_usize() {
        assert_eq!(format_number("12345", 3), "012_345"); // explicit usize 3
        assert_eq!(format_number("12345", 4), "0001_2345");
        assert_eq!(format_number("1.23456", 4), "0001_decimal_point_2345_6000");
        assert_eq!(format_number("-1234", 3), "negative_001_234");
    }

    #[test]
    #[should_panic(expected = "group_size must be positive")]
    fn test_format_number_with_zero_usize_panics() {
        format_number("12345", 0);
    }

    #[test]
    #[should_panic(expected = "group_size must be positive")]
    fn test_format_number_with_zero_str_panics() {
        format_number("12345", "0");
    }
}

pub mod number_theory_functions {
    //! Number theory utilities.
    //!
    //! Currently, this module supports Champernowne number generation
    //! with bases **2 through 36**. Bases outside this range will panic.
    //! 
    //! NOTE: To extend support beyond base 36, you would need to enlarge
    //! the digit alphabet (e.g., include lowercase letters for base 62).

    /// Compute the Champernowne number in a given base.
    ///
    /// - `limit`: how many sequential integers to concatenate.
    /// - `base`: can be:
    ///   - `""` → defaults to base 10,
    ///   - a `usize`,
    ///   - a string parseable as a number.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - base < 2 or base > 36,
    /// - base is not an integer (e.g., `"2.5"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_functions::number_theory_functions::champernowne;
    ///
    /// assert_eq!(champernowne(10, ""), "0.12345678910");
    /// assert_eq!(champernowne(10, 2), "0.11011100101110");
    /// assert_eq!(champernowne(10, "10"), "0.12345678910");
    /// ```
    pub fn champernowne<B: IntoBase>(limit: usize, base: B) -> String {
        let b = base.into_base();
        if b < 2 || b > 36 {
            panic!("Base must be between 2 and 36");
        }

        let mut result = String::from("0.");
        for i in 1..=limit {
            result.push_str(&to_base(i, b));
        }
        result
    }

    /// Helper: convert number to a string in base `b`.
    /// Convert number `n` to a string in base `b`.
    ///
    /// Currently supports bases 2–36 using digits 0–9 and A–Z.
    /// To extend this to larger bases, expand the digit set.
    fn to_base(mut n: usize, b: usize) -> String {
        let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
        let mut res = Vec::new();
        while n > 0 {
            res.push(digits[n % b] as char);
            n /= b;
        }
        if res.is_empty() {
            res.push('0');
        }
        res.iter().rev().collect()
    }

    /// Trait to support `usize`, `&str`, and `String` as base argument
    pub trait IntoBase {
        fn into_base(self) -> usize;
    }

    impl IntoBase for usize {
        fn into_base(self) -> usize {
            self
        }
    }

    impl IntoBase for &str {
        fn into_base(self) -> usize {
            if self.is_empty() {
                return 10;
            }
            self.parse::<usize>().expect("Base must be an integer")
        }
    }

    impl IntoBase for String {
        fn into_base(self) -> usize {
            self.as_str().into_base()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::number_theory_functions::champernowne;

    #[test]
    fn test_champernowne_base10() {
        assert_eq!(
            champernowne(50, 10),
            "0.12345678910111213141516171819202122232425262728293"
        );
    }

    #[test]
    fn test_champernowne_empty_string_defaults_to_10() {
        assert_eq!(
            champernowne(50, ""),
            "0.12345678910111213141516171819202122232425262728293"
        );
    }

    #[test]
    fn test_champernowne_base2() {
        assert_eq!(
            champernowne(50, 2),
            "0.11011100101110111100010011010101111001101111011111"
        );
    }

    // --- BAD CASES ---
    #[test]
    #[should_panic(expected = "Base must be between 2 and 36")]
    fn test_champernowne_base0() {
        champernowne(50, 0);
    }

    #[test]
    #[should_panic(expected = "Base must be between 2 and 36")]
    fn test_champernowne_negative_base_panics() {
        champernowne(50, "-2");
    }

    #[test]
    #[should_panic] // will panic because "2.5" is not parseable as usize
    fn test_champernowne_float_string_panics() {
        champernowne(50, "2.5");
    }

    #[test]
    #[should_panic(expected = "Base must be between 2 and 36")]
    fn test_champernowne_base_too_large_panics() {
        champernowne(50, 100);
    }
}