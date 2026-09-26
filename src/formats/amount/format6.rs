use crate::formats::amount::AmountFormat;
use regex::Regex;

/// Format6: parses amounts like "- $1,234.56", "+ $1,234.56"
pub struct Format6;

impl AmountFormat for Format6 {
    fn num_items(&self) -> usize {
        2
    }

    fn parse(&self, amount_str: &str, num_items: usize) -> Option<f64> {
        if num_items != self.num_items() {
            return None;
        }
        let re = Regex::new(r"^([+-])\s+\$([\d,]+\.\d{2})$").unwrap();
        if !re.is_match(amount_str) {
            return None;
        }

        let caps = re.captures(amount_str).unwrap();
        let sign = &caps[1];
        let cleaned = caps[2].replace(",", "");

        match cleaned.parse::<f64>() {
            Ok(val) => Some(if sign == "-" { -val } else { val }),
            Err(_) => None,
        }
    }
}

// Example usage:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format6() {
        let fmt = Format6;
        assert_eq!(fmt.parse("- $1,234.56", 2), Some(-1234.56));
        assert_eq!(fmt.parse("+ $1,234.56", 2), Some(1234.56));
        assert_eq!(fmt.parse("+ 1,234.56", 2), None);
        assert_eq!(fmt.parse("-$1,234.56", 2), None);
        assert_eq!(fmt.parse("+ $1,234.56", 2), Some(1234.56));
        assert_eq!(fmt.parse("bad input", 2), None);
        assert_eq!(fmt.parse("$1,234.56", 2), None);
        assert_eq!(fmt.parse("1,234.56", 2), None);
        assert_eq!(fmt.parse("- $1234.5", 2), None);
    }
}
