use crate::formats::amount::AmountFormat;
use regex::Regex;

/// Format6: parses amounts like "- $1,234.56", "+ 1,234.56"
pub struct Format6;

impl AmountFormat for Format6 {
    fn num_items(&self) -> usize {
        2
    }

    fn parse(&self, amount_str: &str) -> Option<f64> {
        let re = Regex::new(r"^([+-])\s*\$?([\d,]+\.\d{2})$").unwrap();
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
        assert_eq!(fmt.parse("- $1,234.56"), Some(-1234.56));
        assert_eq!(fmt.parse("+ 1,234.56"), Some(1234.56));
        assert_eq!(fmt.parse("-$1,234.56"), Some(-1234.56));
        assert_eq!(fmt.parse("+ $1,234.56"), Some(1234.56));
        assert_eq!(fmt.parse("bad input"), None);
        assert_eq!(fmt.parse("$1,234.56"), None);
        assert_eq!(fmt.parse("1,234.56"), None);
        assert_eq!(fmt.parse("- $1234.5"), None);
    }
}
