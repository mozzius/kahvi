//! Number <-> string conversions shared by the parser and the engine.

/// Number::toString(x, 10) (ECMA-262 §6.1.6.1.20).
///
/// Rust's shortest round-trip formatting gives the digits `s` and exponent
/// required by step 5 (k as small as possible, closest value on ties); this
/// function then lays them out the way the spec says.
pub fn number_to_string(x: f64) -> String {
    // 1. If x is NaN, return "NaN".
    if x.is_nan() {
        return "NaN".into();
    }
    // 2. If x is either +0𝔽 or -0𝔽, return "0".
    if x == 0.0 {
        return "0".into();
    }
    // 3. If x < -0𝔽, return the string-concatenation of "-" and Number::toString(-x, radix).
    if x < 0.0 {
        return format!("-{}", number_to_string(-x));
    }
    // 4. If x is +∞𝔽, return "Infinity".
    if x.is_infinite() {
        return "Infinity".into();
    }
    // 5. Let n, k, and s be integers such that k ≥ 1, radix^(k-1) ≤ s < radix^k,
    //    𝔽(s × radix^(n - k)) is x, and k is as small as possible.
    let (digits, n) = shortest_digits(x);
    let k = digits.len() as i32;
    // 6. If radix ≠ 10 or n is in the inclusive interval from -5 to 21, then
    if k <= n && n <= 21 {
        // a. If n ≥ k, then: the k digits of s followed by n - k zeros.
        let mut out = digits;
        out.extend(std::iter::repeat_n('0', (n - k) as usize));
        return out;
    }
    if 0 < n && n <= 21 {
        // b. Else if n > 0: the most significant n digits, ".", the remaining k - n digits.
        return format!("{}.{}", &digits[..n as usize], &digits[n as usize..]);
    }
    if -6 < n && n <= 0 {
        // c. Else: "0.", -n zeros, the k digits.
        return format!("0.{}{}", "0".repeat((-n) as usize), digits);
    }
    // 7-12. Exponential notation.
    let exponent = n - 1;
    let sign = if exponent >= 0 { '+' } else { '-' };
    if k == 1 {
        format!("{digits}e{sign}{}", exponent.abs())
    } else {
        format!("{}.{}e{sign}{}", &digits[..1], &digits[1..], exponent.abs())
    }
}

/// The shortest decimal digits of a finite positive x, and n such that
/// x = 0.digits × 10^n.
fn shortest_digits(x: f64) -> (String, i32) {
    let formatted = format!("{x:e}");
    let (mantissa, exp) = formatted
        .split_once('e')
        .expect("LowerExp always has an exponent");
    let exp: i32 = exp.parse().expect("integer exponent");
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    let digits = digits.trim_end_matches('0').to_string();
    (
        if digits.is_empty() {
            "0".into()
        } else {
            digits
        },
        exp + 1,
    )
}

/// Converts an unsigned integer given as digits in `radix` (2, 8, 10 or 16)
/// to the nearest f64, rounding ties to even.
pub fn radix_digits_to_f64(digits: &str, radix: u32) -> f64 {
    if radix == 10 {
        return digits.parse().unwrap_or(f64::NAN);
    }
    let bits_per_digit = radix.trailing_zeros();
    let mut bits: Vec<u8> = Vec::new();
    for c in digits.chars() {
        let d = c.to_digit(radix).unwrap_or(0);
        for i in (0..bits_per_digit).rev() {
            bits.push(((d >> i) & 1) as u8);
        }
    }
    let first_one = match bits.iter().position(|&b| b == 1) {
        Some(i) => i,
        None => return 0.0,
    };
    let bits = &bits[first_one..];
    if bits.len() <= 53 {
        return bits.iter().fold(0u64, |acc, &b| acc << 1 | b as u64) as f64;
    }
    // Keep 53 significant bits and round the rest half-to-even.
    let mut mantissa = bits[..53].iter().fold(0u64, |acc, &b| acc << 1 | b as u64);
    let round = bits[53] == 1;
    let sticky = bits[54..].contains(&1);
    if round && (sticky || mantissa & 1 == 1) {
        mantissa += 1;
    }
    let shift = (bits.len() - 53) as i32;
    (mantissa as f64) * 2f64.powi(shift)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Number::toString is load-bearing for property keys and string
    // conversion everywhere, and its layout rules are easy to get subtly wrong.
    #[test]
    fn number_to_string_layouts() {
        for (x, s) in [
            (1.0, "1"),
            (0.1, "0.1"),
            (123.456, "123.456"),
            (1e21, "1e+21"),
            (1e20, "100000000000000000000"),
            (1e-6, "0.000001"),
            (1e-7, "1e-7"),
            (1.5e-7, "1.5e-7"),
            (-0.0, "0"),
            (-2.5, "-2.5"),
            (f64::INFINITY, "Infinity"),
            (2f64.powi(53), "9007199254740992"),
            (1.7976931348623157e308, "1.7976931348623157e+308"),
            (5e-324, "5e-324"),
        ] {
            assert_eq!(number_to_string(x), s, "{x:e}");
        }
    }

    #[test]
    fn radix_rounding() {
        assert_eq!(radix_digits_to_f64("ff", 16), 255.0);
        assert_eq!(
            radix_digits_to_f64("20000000000001", 16),
            9007199254740992.0
        );
        assert_eq!(
            radix_digits_to_f64("20000000000003", 16),
            9007199254740996.0
        );
    }
}
