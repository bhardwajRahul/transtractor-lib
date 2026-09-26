use crate::formats::amount::AmountFormat;
use regex::Regex;

/// Format7: parses amounts like "-$1,234.56⧫", "$1,234.56⧫", "$1,234.56-⧫"
pub struct Format7;

impl AmountFormat for Format7 {
    fn num_items(&self) -> usize {
        1
    }

    fn parse(&self, currency_str: &str, num_items: usize) -> Option<f64> {
        if num_items != self.num_items() {
            return None;
        }
        let trimmed = currency_str.trim();
        if !trimmed.ends_with('⧫') {
            return None;
        }

        let candidate = trimmed.strip_suffix('⧫').unwrap().trim();
        let re = Regex::new(r"^-?\$\d{1,3}(,\d{3})*\.\d{2}(-|\s)?$").unwrap();
        if !re.is_match(candidate) {
            return None;
        }

        let mut cleaned = candidate.replace(',', "");
        let mut sign = 1.0;
        if cleaned.contains('-') {
            sign = -1.0;
            cleaned = cleaned.replace('-', "");
        }
        cleaned = cleaned.replace('$', "");

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
    fn test_format7() {
        let fmt = Format7;
        assert_eq!(fmt.parse("$1,234.56⧫", 1), Some(1234.56));
        assert_eq!(fmt.parse("-$1,234.56⧫", 1), Some(-1234.56));
        assert_eq!(fmt.parse("$1,234.56-⧫", 1), Some(-1234.56));
        assert_eq!(fmt.parse("bad input", 1), None);
        assert_eq!(fmt.parse("$1,234.5⧫", 1), None);
        assert_eq!(fmt.parse("$1,234.567⧫", 1), None);
        assert_eq!(fmt.parse("$1,000,234.56⧫", 1), Some(1000234.56));
        assert_eq!(fmt.parse("$1,234.56", 1), None);
    }
}
