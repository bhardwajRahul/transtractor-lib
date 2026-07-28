use crate::structs::ProtoTransaction;
use chrono::{DateTime, Datelike, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatementData {
    pub key: Option<String>,
    pub account_number: Option<String>,
    pub start_date: Option<i64>,
    pub start_date_year: Option<i32>,
    pub opening_balance: Option<f64>,
    pub closing_balance: Option<f64>,
    pub proto_transactions: Vec<ProtoTransaction>,
    pub errors: Vec<String>,
}

impl StatementData {
    pub fn new() -> Self {
        Self {
            key: None,
            account_number: None,
            start_date: None,
            start_date_year: None,
            opening_balance: None,
            closing_balance: None,
            proto_transactions: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn account_number(&self) -> Option<&String> {
        self.account_number.as_ref()
    }
    pub fn opening_balance(&self) -> Option<f64> {
        self.opening_balance
    }
    pub fn closing_balance(&self) -> Option<f64> {
        self.closing_balance
    }
    pub fn start_date(&self) -> Option<i64> {
        self.start_date
    }
    pub fn start_date_year(&self) -> Option<i32> {
        self.start_date_year
    }

    // Setters for the fields
    pub fn set_key(&mut self, key: String) {
        self.key = Some(key);
    }

    pub fn set_account_number(&mut self, account_number: String) {
        self.account_number = Some(account_number);
    }

    pub fn set_start_date(&mut self, date: i64) {
        self.start_date = Some(date);
        self.start_date_year = Utc.timestamp_millis_opt(date).single().map(|dt| dt.year());
    }

    pub fn set_opening_balance(&mut self, balance: f64) {
        self.opening_balance = Some(balance);
    }

    pub fn set_closing_balance(&mut self, balance: f64) {
        self.closing_balance = Some(balance);
    }

    pub fn add_proto_transaction(&mut self, proto_tx: ProtoTransaction) {
        self.proto_transactions.push(proto_tx);
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn print(&self) {
        println!("{}", self);
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn matches(&self, other: &Self) -> bool {
        self.key == other.key
            && self.account_number == other.account_number
            && self.start_date == other.start_date
            && self.start_date_year == other.start_date_year
            && self.opening_balance == other.opening_balance
            && self.closing_balance == other.closing_balance
            && self.proto_transactions == other.proto_transactions
            && self.errors == other.errors
    }
}

impl fmt::Display for StatementData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str("Statement Data:\n");
        match &self.key {
            Some(k) => result.push_str(&format!("  Key: {}\n", k)),
            None => result.push_str("  Key: Not set\n"),
        }
        match &self.account_number {
            Some(an) => result.push_str(&format!("  Account Number: {}\n", an)),
            None => result.push_str("  Account Number: Not set\n"),
        }
        if let Some(ms) = self.start_date {
            if let Some(dt) = DateTime::<Utc>::from_timestamp_millis(ms) {
                result.push_str(&format!("  Start Date: {}\n", dt.format("%d %b %Y")));
            } else {
                result.push_str(&format!("  Start Date: {}\n", ms));
            }
        } else {
            result.push_str("  Start Date: Not set\n");
        }
        if let Some(balance) = self.opening_balance {
            result.push_str(&format!("  Opening Balance: {:.2}\n", balance));
        } else {
            result.push_str("  Opening Balance: Not set\n");
        }
        if let Some(balance) = self.closing_balance {
            result.push_str(&format!("  Closing Balance: {:.2}\n", balance));
        } else {
            result.push_str("  Closing Balance: Not set\n");
        }
        result.push_str("  Proto Transactions:\n");
        for (i, tx) in self.proto_transactions.iter().enumerate() {
            let date_str = match tx.date {
                Some(ms) => match DateTime::<Utc>::from_timestamp_millis(ms) {
                    Some(dt) => dt.format("%d %b %Y").to_string(),
                    None => ms.to_string(),
                },
                None => "Not set".to_string(),
            };
            let amount_str = match tx.amount {
                Some(a) => format!("{:.2}", a),
                None => "Not set".to_string(),
            };
            let balance_str = match tx.balance {
                Some(b) => format!("{:.2}", b),
                None => "Not set".to_string(),
            };
            result.push_str(&format!(
                "    {}: {}, \"{}\", {}, {}\n",
                i + 1,
                date_str,
                tx.description,
                amount_str,
                balance_str
            ));
        }
        if !self.errors.is_empty() {
            result.push_str("  Errors:\n");
            for error in &self.errors {
                result.push_str(&format!("    - {}\n", error));
            }
        } else {
            result.push_str("  Errors: None\n");
        }
        write!(f, "{}", result)
    }
}

impl Default for StatementData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialises_statement_data_to_json() {
        let mut data = StatementData::new();
        data.set_key("statement-1".to_string());
        data.set_account_number("ACC-123".to_string());
        data.set_start_date(1_700_000_000_000i64);
        data.set_opening_balance(100.5);
        data.set_closing_balance(200.25);
        data.add_proto_transaction(ProtoTransaction {
            date: Some(1_700_000_000_000i64),
            index: 1,
            description: "Coffee".to_string(),
            amount: Some(4.5),
            balance: Some(104.0),
        });
        data.add_error("warning".to_string());

        let json = data.to_json().unwrap();

        assert!(json.contains("\"key\":\"statement-1\""));
        assert!(json.contains("\"account_number\":\"ACC-123\""));
        assert!(json.contains("\"description\":\"Coffee\""));
    }

    #[test]
    fn deserialises_statement_data_from_json() {
        let json = r#"{
            "key": "statement-2",
            "account_number": "ACC-999",
            "start_date": 1700000000000,
            "start_date_year": 2023,
            "opening_balance": 15.75,
            "closing_balance": 20.5,
            "proto_transactions": [
                {
                    "date": 1700000000000,
                    "index": 2,
                    "description": "Taxi",
                    "amount": 5.25,
                    "balance": 20.5
                }
            ],
            "errors": ["review needed"]
        }"#;

        let data = StatementData::from_json(json).unwrap();

        assert_eq!(data.key.as_deref(), Some("statement-2"));
        assert_eq!(data.account_number.as_deref(), Some("ACC-999"));
        assert_eq!(data.start_date, Some(1_700_000_000_000i64));
        assert_eq!(data.opening_balance, Some(15.75));
        assert_eq!(data.closing_balance, Some(20.5));
        assert_eq!(data.proto_transactions.len(), 1);
        assert_eq!(data.proto_transactions[0].description, "Taxi");
        assert_eq!(data.errors, vec!["review needed".to_string()]);
    }

    #[test]
    fn compares_statement_data_objects() {
        let mut first = StatementData::new();
        first.set_key("statement-1".to_string());
        first.set_account_number("ACC-123".to_string());
        first.add_proto_transaction(ProtoTransaction {
            date: Some(1_700_000_000_000i64),
            index: 1,
            description: "Coffee".to_string(),
            amount: Some(4.5),
            balance: Some(104.0),
        });

        let mut second = StatementData::new();
        second.set_key("statement-1".to_string());
        second.set_account_number("ACC-123".to_string());
        second.add_proto_transaction(ProtoTransaction {
            date: Some(1_700_000_000_000i64),
            index: 1,
            description: "Coffee".to_string(),
            amount: Some(4.5),
            balance: Some(104.0),
        });

        let mut third = StatementData::new();
        third.set_key("statement-2".to_string());

        assert!(first.matches(&second));
        assert!(!first.matches(&third));
    }
}
