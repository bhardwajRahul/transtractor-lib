use crate::structs::TextItem;

/// Converts a collection of TextItems into a structured layout text format
pub fn text_items_to_layout(items: &Vec<TextItem>) -> Result<String, String> {
    if items.is_empty() {
        return Ok(String::new());
    }

    let mut output = String::new();
    let mut current_page = items[0].page;
    let mut last_y1_bin = items[0].y1_bin;

    // Start with the first page marker
    output.push_str(&format!("[Page {}]\n", current_page));

    for item in items {
        // Check if we're on a new page
        if item.page != current_page {
            current_page = item.page;
            output.push_str(&format!("\n[Page {}]\n", current_page));
            last_y1_bin = item.y1_bin;
        } else {
            if item.y1_bin != last_y1_bin {
                output.push('\n');
                last_y1_bin = item.y1_bin;
            }
        }

        // Print the item in the format [text, x1, x2, y1, y2, y1_bin]
        output.push_str(&format!(
            "[\"{}\",{},{},{},{},{}]",
            item.text, item.x1, item.x2, item.y1, item.y2, item.y1_bin
        ));
    }

    Ok(output)
}
