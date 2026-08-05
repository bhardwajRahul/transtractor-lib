use crate::checkers::check_statement_data;
use crate::fixers::fix_statement_data;
use crate::parsers::top::parse_text_items;
use crate::structs::StatementConfig;
use crate::structs::StatementData;
use crate::structs::TextItem;
use crate::structs::text_items::tokenise_items;

/// Extract StatementData objects from text items using provided statement configurations.
pub fn text_items_to_statement_datas(
    items: &Vec<TextItem>,
    configs: &Vec<StatementConfig>,
    exit_when_succeed: bool,
) -> Result<Vec<StatementData>, String> {
    let mut results = Vec::new();
    for cfg in configs {
        let tokenised_items = tokenise_items(items);
        let mut data = parse_text_items(cfg, &tokenised_items);
        data.set_key(cfg.key.clone());

        // Apply fixers to clean up the data
        fix_statement_data(&mut data);
        check_statement_data(&mut data);

        // Return early if an error-free StatementData is found
        if exit_when_succeed && data.errors.is_empty() {
            results.push(data);
            break;
        }

        results.push(data);
    }

    Ok(results)
}
