use crate::checkers::check_statement_data;
use crate::fixers::fix_statement_data;
use crate::parsers::flows::text_items_to_statement_data::text_items_to_statement_data;
use crate::structs::StatementConfig;
use crate::structs::StatementData;
use crate::structs::TextItem;
use crate::structs::text_items::tokenise_items;

/// Parse non-tokenised text items into list of statement data results,
/// using provided statement configurations.
pub fn text_items_to_statement_datas(
    items: &Vec<TextItem>,
    configs: &Vec<StatementConfig>,
) -> Result<Vec<StatementData>, String> {
    let mut results = Vec::new();
    for cfg in configs {
        let tokenised_items = tokenise_items(items);
        let mut data = text_items_to_statement_data(cfg, &tokenised_items);
        data.set_key(cfg.key.clone());

        // Apply fixers to clean up the data
        fix_statement_data(&mut data);
        check_statement_data(&mut data);

        results.push(data);
    }

    Ok(results)
}
