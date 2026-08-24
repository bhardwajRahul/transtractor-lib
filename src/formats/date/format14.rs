use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format14: parses MM/DD/YYYY* or MM/DD/YY* dates like "03/24/2023*", "3/24/2023*", "03/24/23*", "3/24/23*"
pub struct Format14;

impl DateFormat for Format14 {
    fn num_items(&self) -> usize {
        1
    }

    fn parse(&self, date_str: &str, _year_str: &str) -> Option<i64> {
        let date_str = date_str.strip_suffix('*')?;
        let re = regex::Regex::new(r"^\d{1,2}/\d{1,2}/\d{2,4}$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        let parts: Vec<&str> = date_str.split('/').collect();
        if parts.len() != 3 {
            return None;
        }
        let date_parts = DateParts {
            day_str: parts[1].to_string(),
            month_str: parts[0].to_string(),
            year_str: parts[2].to_string(),
        };
        date_parts.to_utc_timestamp("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format14_parse() {
        let fmt = Format14;
        assert!(fmt.parse("03/24/2023*", "").is_some());
        assert!(fmt.parse("3/24/2023*", "").is_some());
        assert!(fmt.parse("03/24/23*", "").is_some());
        assert!(fmt.parse("3/24/23*", "").is_some());
        assert!(fmt.parse("1/1/2023*", "").is_some());
        assert!(fmt.parse("02/30/2023*", "").is_none());
        assert_eq!(fmt.parse("03/24/2023", ""), None);
        assert_eq!(fmt.parse("03/24/2023**", ""), None);
        assert_eq!(fmt.parse("03/24/2023* ", ""), None);
        assert_eq!(fmt.parse("03-24-2023*", ""), None);
    }
}
