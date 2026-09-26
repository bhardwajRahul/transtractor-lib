use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format4: parses dates like "24/3/2020", "01/03/2020", "24/03/2020"
pub struct Format4;

impl DateFormat for Format4 {
    fn num_items(&self) -> usize {
        1
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    fn parse(&self, date_str: &str, _year_str: &str, num_items: usize) -> Option<i64> {
        if num_items != self.num_items() {
            return None;
        }
        let re = regex::Regex::new(r"^\d{1,2}/\d{1,2}/\d{4}$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        let parts: Vec<&str> = date_str.split('/').collect();
        if parts.len() != 3 {
            return None;
        }
        let date_parts = DateParts {
            day_str: parts[0].to_string(),
            month_str: parts[1].to_string(),
            year_str: parts[2].to_string(),
        };
        date_parts.to_utc_timestamp("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format4_parse() {
        let fmt = Format4;
        // "24/3/2020"
        let ts = fmt.parse("24/3/2020", "", 1);
        assert!(ts.is_some());
        // "01/03/2020"
        let ts2 = fmt.parse("01/03/2020", "", 1);
        assert!(ts2.is_some());
        // "24/03/2020"
        let ts3 = fmt.parse("24/03/2020", "", 1);
        assert!(ts3.is_some());
        // Invalid
        assert_eq!(fmt.parse("24-03-2020", "", 1), None);
        assert_eq!(fmt.parse("24/03", "", 1), None);
        assert_eq!(fmt.parse("", "", 1), None);
    }
}
