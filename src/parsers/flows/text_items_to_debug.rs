use crate::configs::db::ConfigDB;
use crate::parsers::flows::text_items_to_statement_datas::text_items_to_statement_datas;
use crate::structs::TextItem;

/// Parse non-tokenised text items into debug information string,
/// using provided statement configurations.
pub fn text_items_to_debug(config_db: &ConfigDB, items: &Vec<TextItem>) -> Result<String, String> {
    let configs = config_db.identify(items);

    // User error: trying to parse unsupported bank statement format
    if configs.is_empty() {
        return Err("Bank statement format cannot be identified.".to_string());
    }

    // Write debug information to the output file
    let mut output = String::new();
    output.push_str("Debug output\n");

    match text_items_to_statement_datas(items, &configs, false) {
        Ok(statement_data_results) => {
            output.push_str(&format!(
                "Found {} StatementData result(s)\n\n",
                statement_data_results.len()
            ));

            for (i, data) in statement_data_results.iter().enumerate() {
                output.push_str(&format!("=== StatementData Result {} ===\n", i + 1));
                output.push_str(&data.to_string());
                output.push('\n');
            }
        }
        Err(e) => {
            return Err(format!("Unexpected error: {}", e));
        }
    }
    Ok(output)
}
