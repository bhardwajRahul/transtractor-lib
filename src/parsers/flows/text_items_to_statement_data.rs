use crate::configs::db::ConfigDB;
use crate::parsers::flows::text_items_to_statement_datas::text_items_to_statement_datas;
use crate::structs::{StatementData, TextItem};

/// Top-level workflow to parse extracted text items into structured statement data
pub fn text_items_to_statement_data(
    config_db: &ConfigDB,
    items: &Vec<TextItem>,
) -> Result<StatementData, String> {
    let configs = config_db.identify(items);

    // User error: trying to parse unsupported bank statement format
    if configs.is_empty() {
        return Err("Bank statement format cannot be identified.".to_string());
    }

    // Return first error-free StatementData
    let statement_data_results = text_items_to_statement_datas(items, &configs, true)?;
    for data in statement_data_results {
        if data.errors.is_empty() {
            return Ok(data);
        }
    }

    // Software bug: If statement is recognised, it should be parsed successfully
    let keys: Vec<String> = configs.iter().map(|cfg| cfg.key.clone()).collect();
    Err(format!(
        "Bank statement recognised but cannot be parsed. Debug configurations: {:?}",
        keys
    ))
}
