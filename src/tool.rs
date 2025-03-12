use std::str::FromStr;

use eyre::{Context, eyre};
use num_bigint::BigInt;
use rand::Rng;

pub fn parse_bn(bn: u64, decimals: u8) -> f64 {
    bn as f64 / 10u64.pow(decimals as u32) as f64
}

pub fn parse_units(value: &str, decimals: u8) -> eyre::Result<BigInt> {
    let value = value.replace(",", "");
    let value = value.replace(" ", "");
    let value = value.replace("_", "");

    let mut parts: Vec<&str> = value.split('.').collect();
    match parts.len() {
        1 => {
            let mut number = BigInt::from_str(parts[0]).context("invalid decimal string")?;
            number *= BigInt::from(10).pow(decimals as u32);
            Ok(number)
        }
        2 => {
            let whole = BigInt::from_str(parts[0]).context("invalid decimal string")?
                * BigInt::from(10).pow(decimals as u32);
            let mut decimal_places = parts[1].len();

            if decimal_places > decimals as usize {
                parts[1] = &parts[1][..decimals as usize];
                decimal_places = decimals as usize;
            }

            let decimal = BigInt::from_str(parts[1]).context("invalid decimal string")?
                * BigInt::from(10).pow((decimals - decimal_places as u8) as u32);
            Ok(whole + decimal)
        }
        _ => Err(eyre!("invalid decimal string")),
    }
}

pub fn format_units(value: &BigInt, decimals: u8) -> String {
    let divisor = BigInt::from(10).pow(decimals as u32);

    let quotient = value / &divisor;
    let remainder = value % &divisor;

    let mut result = quotient.to_string();
    if remainder != BigInt::from(0) {
        let mut decimal = remainder.to_string();
        decimal = "0".repeat(decimals as usize - decimal.len()) + &decimal;
        decimal = decimal.trim_end_matches('0').to_string();
        if !decimal.is_empty() {
            result.push('.');
            result.push_str(&decimal);
        }
    }

    result
}
pub fn add_slippage(amount: &BigInt, slippage_percent: f64) -> BigInt {
    let slippage = slippage_percent / 100.0;
    let factor = 10_000u64;
    let slippage: u64 = (slippage * factor as f64).round() as u64;

    let factor = BigInt::from(factor);
    let slippage = BigInt::from(slippage);

    amount * (factor.clone() + slippage) / factor.clone()
}

pub fn sub_slippage(amount: &BigInt, slippage_percent: f64) -> BigInt {
    let slippage = slippage_percent / 100.0;
    let factor = 10_000u64;
    let slippage: u64 = (slippage * factor as f64).round() as u64;

    let factor = BigInt::from(factor);
    let slippage = BigInt::from(slippage);

    amount * (factor.clone() - slippage) / factor.clone()
}

pub fn random(min: &BigInt, max: &BigInt) -> BigInt {
    if min > max {
        panic!("min must be less than max");
    }
    let mut rng = rand::thread_rng();
    let range = max - min;
    let limit = 1_000_000_000;
    let random = rng.gen_range(0..limit);
    min + range * BigInt::from(random) / BigInt::from(limit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_units() {
        assert_eq!(
            parse_units("1", 18).unwrap().to_string(),
            "1000000000000000000"
        );
        assert_eq!(
            parse_units("1.0", 18).unwrap().to_string(),
            "1000000000000000000"
        );
        assert_eq!(
            parse_units("1.1", 18).unwrap().to_string(),
            "1100000000000000000"
        );
        assert_eq!(
            parse_units("0.1", 18).unwrap().to_string(),
            "100000000000000000"
        );
        assert_eq!(
            parse_units("1_000_000", 2).unwrap().to_string(),
            "100000000"
        );
        assert!(parse_units("1.1.1", 18).is_err());
        assert!(parse_units("abc", 18).is_err());
    }

    #[test]
    fn test_format_units() {
        let value = BigInt::from_str("1000000000000000000").unwrap();
        assert_eq!(format_units(&value, 18), "1");

        let value = BigInt::from_str("1100000000000000000").unwrap();
        assert_eq!(format_units(&value, 18), "1.1");

        let value = BigInt::from_str("100000000000000000").unwrap();
        assert_eq!(format_units(&value, 18), "0.1");

        let value = BigInt::from_str("1234567890123456789").unwrap();
        assert_eq!(format_units(&value, 18), "1.234567890123456789");

        let value = BigInt::from_str("1000000000000000000000").unwrap();
        assert_eq!(format_units(&value, 18), "1000");
    }
    use num_bigint::BigInt;

    #[test]
    fn test_random_within_bounds() {
        let min = BigInt::from(0);
        let max = BigInt::from(100);

        for _ in 0..1000 {
            let result = random(&min, &max);
            assert!(result >= min);
            assert!(result <= max);
        }
    }

    #[test]
    fn test_random_negative_range() {
        let min = BigInt::from(-100);
        let max = BigInt::from(100);

        for _ in 0..1000 {
            let result = random(&min, &max);
            assert!(result >= min);
            assert!(result <= max);
        }
    }

    #[test]
    fn test_random_large_numbers() {
        let min = BigInt::parse_bytes(b"1000000000000000000000", 10).unwrap();
        let max = BigInt::parse_bytes(b"1000000000000000000100", 10).unwrap();

        for _ in 0..100 {
            let result = random(&min, &max);
            assert!(result >= min);
            assert!(result <= max);
        }
    }

    #[test]
    fn test_random_same_bounds() {
        let value = BigInt::from(42);
        let result = random(&value, &value);
        assert_eq!(result, value);
    }

    #[test]
    #[should_panic]
    fn test_invalid_bounds() {
        let min = BigInt::from(100);
        let max = BigInt::from(0);
        random(&min, &max);
    }

    #[test]
    fn test_distribution() {
        let min = BigInt::from(0);
        let max = BigInt::from(10);
        let mut counts = vec![0; 10];
        let iterations = 10000;

        for _ in 0..iterations {
            let result = random(&min, &max);
            let result: u64 = result.try_into().unwrap();
            let index = result as usize;
            counts[index] += 1;
        }

        // Check if each number appears at least 5% of the time
        dbg!(&counts);
        let min_expected = iterations / 20;
        for count in counts {
            assert!(count >= min_expected);
        }
    }
}
