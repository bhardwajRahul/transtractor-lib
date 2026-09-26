use crate::formats::amount::AmountFormat;
use regex::Regex;

/// Format3: parses amounts like "-$1,234.56 DR", "$1,234.56 DR", "$1,234.56 CR"
pub struct Format3;

impl AmountFormat for Format3 {
    fn num_items(&self) -> usize {
        2
    }

    fn parse(&self, currency_str: &str, num_items: usize) -> Option<f64> {
        if num_items != self.num_items() {
            return None;
        }
        let currency_str = currency_str.to_lowercase();
        let re = Regex::new(r"^-?\$\d{1,3}(,\d{3})*\.\d{2} (cr|dr)$").unwrap();
        if !re.is_match(&currency_str) {
            return None;
        }
        let mut sign = 1.0;
        if currency_str.contains("dr") {
            sign = -1.0;
        }
        // Remove "cr" or "dr"
        let mut cleaned = Regex::new(r"(cr|dr)")
            .unwrap()
            .replace(&currency_str, "")
            .to_string();
        if cleaned.contains('-') {
            sign *= -1.0;
            cleaned = cleaned.replace('-', "");
        }
        cleaned = cleaned.replace(['$', ','], "").trim().to_string();
        match cleaned.parse::<f64>() {
            Ok(val) => Some(sign * val),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format3() {
        let fmt = Format3;
        assert_eq!(fmt.parse("$1,234.56 DR", 2), Some(-1234.56));
        assert_eq!(fmt.parse("-$1,234.56 DR", 2), Some(1234.56));
        assert_eq!(fmt.parse("$1,234.56 CR", 2), Some(1234.56));
        assert_eq!(fmt.parse("bad input", 2), None);
        assert_eq!(fmt.parse("1234.56 DR", 2), None);
        assert_eq!(fmt.parse("$1234.56 DR", 2), None);
        assert_eq!(fmt.parse("$1,234.5 DR", 2), None);
        assert_eq!(fmt.parse("$1,234.567 DR", 2), None);
        assert_eq!(fmt.parse("$1,000,234.56 CR", 2), Some(100_0234.56));
        assert_eq!(fmt.parse("$4.00 DR", 2), Some(-4.00));
    }
}
