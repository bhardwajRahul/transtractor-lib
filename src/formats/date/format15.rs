use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format15: parses dates like "Jul 1 - Jul 31, 2026"
pub struct Format15;

impl DateFormat for Format15 {
    fn num_items(&self) -> usize {
        6
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    fn parse(&self, date_str: &str, _year_str: &str) -> Option<i64> {
        let re = regex::Regex::new(r"^\w+ \d{1,2} - \w+ \d{1,2}, \d{4}$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        // Remove comma and split
        let cleaned = date_str.replace(",", "");
        let parts: Vec<&str> = cleaned.split(' ').collect();
        if parts.len() != 6 {
            return None;
        }
        let date_parts = DateParts {
            day_str: parts[1].to_string(),
            month_str: parts[0].to_string(),
            year_str: parts[5].to_string(),
        };
        date_parts.to_utc_timestamp("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format15_parse() {
        let fmt = Format15;
        assert!(fmt.parse("Jul 1 - Jul 31, 2026", "").is_some());
        assert!(fmt.parse("January 1 - January 31, 2026", "").is_some());
        assert!(fmt.parse("Jan 1 - Jan 31, 2026", "").is_some());
        assert_eq!(fmt.parse("Jul 1 - Jul 31 2026", ""), None);
        assert_eq!(fmt.parse("Jul 1 - Jul 31, 2026*", ""), None);
        assert_eq!(fmt.parse("Jul 1 - Jul 31, 26", ""), None);
        assert_eq!(fmt.parse("Mar 24, 2023-Apr 24, 2023", ""), None);
    }
}
