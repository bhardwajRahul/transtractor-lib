use crate::formats::amount::AmountFormat;
use crate::formats::amount::format2::Format2;

/// Format8: parses amounts like "-$1,234.56", "$1,234.56", "$1,234.56-"
/// This simply inverts the sign of the parsed amount from Format2 and is purely a hack
/// to parse payments in Amex Platinum statements. These values have a distinct format
/// that can be selectively inverted to ensure correct parsing.
pub struct Format8;

impl AmountFormat for Format8 {
    fn num_items(&self) -> usize {
        1
    }

    fn parse(&self, currency_str: &str) -> Option<f64> {
        Format2.parse(currency_str).map(|value| value * -1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format8() {
        let fmt = Format8;
        assert_eq!(fmt.parse("$1,234.56"), Some(-1234.56));
        assert_eq!(fmt.parse("-$1,234.56"), Some(1234.56));
        assert_eq!(fmt.parse("$1,234.56-"), Some(1234.56));
        assert_eq!(fmt.parse("bad input"), None);
        assert_eq!(fmt.parse("1234.56"), None);
        assert_eq!(fmt.parse("$1234.56"), None);
        assert_eq!(fmt.parse("$1,234.5"), None);
        assert_eq!(fmt.parse("$1,234.567"), None);
        assert_eq!(fmt.parse("$1,000,234.56"), Some(-100_0234.56));
    }
}
