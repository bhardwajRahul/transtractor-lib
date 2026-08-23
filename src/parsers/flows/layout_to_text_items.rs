use crate::structs::TextItem;

fn parse_quoted_text(input: &str) -> Option<(String, usize)> {
    let mut chars = input.char_indices();
    if !matches!(chars.next(), Some((0, '"'))) {
        return None;
    }

    let mut escaped = false;
    let mut text = String::new();

    for (idx, ch) in chars {
        if escaped {
            text.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '\\' => escaped = true,
            '"' => return Some((text, idx + ch.len_utf8())),
            _ => text.push(ch),
        }
    }

    None
}

fn find_layout_item_end(input: &str) -> Option<usize> {
    let mut in_quotes = false;
    let mut escaped = false;

    for (idx, ch) in input.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        match ch {
            '\\' if in_quotes => escaped = true,
            '"' => in_quotes = !in_quotes,
            ']' if !in_quotes => return Some(idx),
            _ => {}
        }
    }

    None
}

fn parse_layout_item(input: &str) -> Option<(String, i32, i32, i32, i32, i32, usize)> {
    let input = input.trim_start();
    if !input.starts_with('[') {
        return None;
    }

    let end = find_layout_item_end(input)?;
    if end == 0 {
        return None;
    }

    let block = &input[..end + 1];
    let contents = block.get(1..block.len() - 1)?;

    let trimmed_contents = contents.trim_start();
    let (text, text_len) = parse_quoted_text(trimmed_contents)?;
    let remainder = &trimmed_contents[text_len..].trim_start();
    let remainder = remainder.trim_start_matches(',').trim_start();

    let values: Vec<i32> = remainder
        .split(',')
        .map(str::trim)
        .map(|value| value.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    if values.len() != 5 {
        return None;
    }

    Some((
        text,
        values[0],
        values[2],
        values[1],
        values[3],
        values[4],
        end + 1,
    ))
}

/// Converts layout text format to a collection of TextItems
pub fn layout_to_text_items(layout_text: &str) -> Result<Vec<TextItem>, String> {
    let mut text_items: Vec<TextItem> = Vec::new();
    let mut current_page = 0;
    let mut cursor = 0;

    while cursor < layout_text.len() {
        let remaining = &layout_text[cursor..];
        let trimmed = remaining.trim_start();
        let trimmed_start = cursor + (remaining.len() - trimmed.len());

        if trimmed.starts_with("[Page") {
            let page_end = trimmed.find(']').unwrap_or(trimmed.len());
            let page_text = trimmed["[Page".len()..page_end].trim();
            if let Ok(page) = page_text.parse::<i32>() {
                current_page = page;
            }
            cursor = trimmed_start + page_end + 1;
            continue;
        }

        if let Some((text, x1, y1, x2, y2, y1_bin, consumed)) = parse_layout_item(trimmed) {
            let mut item = TextItem::new(text, x1, y1, x2, y2, current_page);
            item.y1_bin = y1_bin;
            text_items.push(item);
            cursor = trimmed_start + consumed;
        } else {
            cursor += 1;
        }
    }

    Ok(text_items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::flows::text_items_to_layout::text_items_to_layout;

    #[test]
    fn round_trips_layout_text_back_to_text_items() {
        let items = vec![
            TextItem::new("Alpha".to_string(), 1, 3, 5, 7, 0),
            TextItem::new("Beta".to_string(), 8, 10, 12, 14, 1),
        ];

        let layout = text_items_to_layout(&items).unwrap();
        let parsed = layout_to_text_items(&layout).unwrap();

        assert_eq!(parsed, items);
    }

    #[test]
    fn preserves_y1_bin_during_layout_round_trip() {
        let item = TextItem {
            text: "Gamma".to_string(),
            x1: 1,
            y1: 3,
            x2: 5,
            y2: 7,
            page: 0,
            y1_bin: 99,
        };

        let layout = text_items_to_layout(&vec![item.clone()]).unwrap();
        let parsed = layout_to_text_items(&layout).unwrap();

        assert_eq!(parsed[0], item);
    }

    #[test]
    fn parses_unicode_text_from_layout() {
        let layout = "[Page 0]\n[\"- Pay Over Time and/or Cash Advance activity ⧫\",1,2,3,4,5]";

        let parsed = layout_to_text_items(layout).unwrap();

        assert_eq!(parsed.len(), 1);
        assert_eq!(
            parsed[0].text,
            "- Pay Over Time and/or Cash Advance activity ⧫"
        );
        assert_eq!(parsed[0].x1, 1);
        assert_eq!(parsed[0].x2, 2);
        assert_eq!(parsed[0].y1, 3);
        assert_eq!(parsed[0].y2, 4);
        assert_eq!(parsed[0].y1_bin, 5);
    }

    #[test]
    fn skips_malformed_closing_bracket_without_panicking() {
        let parsed = layout_to_text_items("] [Page 0] [\"Alpha\",1,2,3,4,5]").unwrap();

        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].text, "Alpha");
    }

    #[test]
    fn parses_text_with_closing_bracket_inside_quotes() {
        let parsed = layout_to_text_items("[Page 0]\n[\"Label with ] inside\",1,2,3,4,5]").unwrap();

        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].text, "Label with ] inside");
    }
}
