use crate::formats::amount::AmountFormat;

/// Format5: parses "Nil" or "nil" as 0, anything else as None
pub struct Format5;

impl AmountFormat for Format5 {
    fn num_items(&self) -> usize {
        1
    }

    fn parse(&self, currency_str: &str, num_items: usize) -> Option<f64> {
        if num_items != self.num_items() {
            return None;
        }
        if currency_str.trim().eq_ignore_ascii_case("nil") {
            Some(0.0)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format5() {
        let fmt = Format5;
        assert_eq!(fmt.parse("Nil", 1), Some(0.0));
        assert_eq!(fmt.parse("nil", 1), Some(0.0));
        assert_eq!(fmt.parse(" NIL ", 1), Some(0.0));
        assert_eq!(fmt.parse("none", 1), None);
        assert_eq!(fmt.parse("0", 1), None);
    }
}
