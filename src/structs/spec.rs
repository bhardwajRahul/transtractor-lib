use crate::configs::db::ConfigDB;
use crate::parsers::flows::text_items_to_statement_data::text_items_to_statement_data;
use crate::structs::ProtoTransaction;
use crate::structs::StatementData;
use crate::structs::text_item::TextItem;
use serde::{Deserialize, Serialize};
use serde_json;

const FLOAT_MATCH_TOLERANCE: f64 = 0.01;

fn option_f64_matches(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(l), Some(r)) => (l - r).abs() < FLOAT_MATCH_TOLERANCE,
        (None, None) => true,
        _ => false,
    }
}

fn proto_transaction_matches(left: &ProtoTransaction, right: &ProtoTransaction) -> bool {
    left.date == right.date
        && left.index == right.index
        && left.description == right.description
        && option_f64_matches(left.amount, right.amount)
        && option_f64_matches(left.balance, right.balance)
}

fn proto_transactions_match(left: &[ProtoTransaction], right: &[ProtoTransaction]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right.iter())
            .all(|(l, r)| proto_transaction_matches(l, r))
}

pub fn matches(generated: &StatementData, expected: &StatementData) -> bool {
    generated.key == expected.key
        && generated.account_number == expected.account_number
        && generated.start_date == expected.start_date
        && generated.start_date_year == expected.start_date_year
        && option_f64_matches(generated.opening_balance, expected.opening_balance)
        && option_f64_matches(generated.closing_balance, expected.closing_balance)
        && proto_transactions_match(&generated.proto_transactions, &expected.proto_transactions)
        && generated.errors == expected.errors
}

#[derive(Serialize, Deserialize)]
pub struct Spec {
    pub statement_data: StatementData,
    pub text_items: Vec<TextItem>,
}

impl Spec {
    pub fn new(config_db: &ConfigDB, text_items: Vec<TextItem>) -> Result<Self, String> {
        let sd = text_items_to_statement_data(config_db, &text_items)?;
        Ok(Spec {
            statement_data: sd,
            text_items,
        })
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert!(matches(&first, &second));
        assert!(!matches(&first, &third));
    }

    #[test]
    fn compares_statement_data_with_float_tolerance() {
        let mut first = StatementData::new();
        first.set_opening_balance(100.0);
        first.set_closing_balance(120.0);
        first.add_proto_transaction(ProtoTransaction {
            date: Some(1_700_000_000_000i64),
            index: 1,
            description: "Coffee".to_string(),
            amount: Some(4.5),
            balance: Some(104.0),
        });

        let mut within_tolerance = StatementData::new();
        within_tolerance.set_opening_balance(100.009);
        within_tolerance.set_closing_balance(119.991);
        within_tolerance.add_proto_transaction(ProtoTransaction {
            date: Some(1_700_000_000_000i64),
            index: 1,
            description: "Coffee".to_string(),
            amount: Some(4.509),
            balance: Some(104.009),
        });

        let mut at_tolerance = StatementData::new();
        at_tolerance.set_opening_balance(100.01);
        at_tolerance.set_closing_balance(120.01);
        at_tolerance.add_proto_transaction(ProtoTransaction {
            date: Some(1_700_000_000_000i64),
            index: 1,
            description: "Coffee".to_string(),
            amount: Some(4.51),
            balance: Some(104.01),
        });

        assert!(matches(&first, &within_tolerance));
        assert!(!matches(&first, &at_tolerance));
    }
}
