use crate::configs::db::ConfigDB;
use crate::parsers::flows::text_items_to_statement_data::text_items_to_statement_data;
use crate::structs::StatementData;
use crate::structs::text_item::TextItem;
use serde::{Deserialize, Serialize};
use serde_json;

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
