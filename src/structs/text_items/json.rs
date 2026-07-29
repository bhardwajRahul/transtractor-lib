use crate::structs::text_item::TextItem;

/// Serialises TextItems into JSON format: [[text,x1,x2,y1,y2,page,y1_bin],...]
pub fn text_items_to_json(items: &[TextItem]) -> Result<String, serde_json::Error> {
    let rows: Vec<(String, i32, i32, i32, i32, i32, i32)> = items
        .iter()
        .map(|item| {
            (
                item.text.clone(),
                item.x1,
                item.x2,
                item.y1,
                item.y2,
                item.page,
                item.y1_bin,
            )
        })
        .collect();

    serde_json::to_string(&rows)
}

/// Deserialises TextItems from JSON format: [[text,x1,x2,y1,y2,page,y1_bin],...]
pub fn json_to_text_items(json: &str) -> Result<Vec<TextItem>, String> {
    let rows: Vec<(String, i32, i32, i32, i32, i32, i32)> =
        serde_json::from_str(json).map_err(|error| error.to_string())?;

    Ok(rows
        .into_iter()
        .map(|(text, x1, x2, y1, y2, page, y1_bin)| TextItem {
            text,
            x1,
            y1,
            x2,
            y2,
            page,
            y1_bin,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialises_text_items_to_json() {
        let items = vec![TextItem {
            text: "Alpha".to_string(),
            x1: 10,
            y1: 20,
            x2: 30,
            y2: 40,
            page: 7,
            y1_bin: 25,
        }];

        let json = text_items_to_json(&items).unwrap();
        assert_eq!(json, r#"[["Alpha",10,30,20,40,7,25]]"#);
    }

    #[test]
    fn deserialises_text_items_from_json() {
        let json = r#"[["Alpha",10,30,20,40,7,25],["Beta",1,2,3,4,8,5]]"#;

        let parsed = json_to_text_items(json).unwrap();

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].text, "Alpha");
        assert_eq!(parsed[0].x1, 10);
        assert_eq!(parsed[0].x2, 30);
        assert_eq!(parsed[0].y1, 20);
        assert_eq!(parsed[0].y2, 40);
        assert_eq!(parsed[0].page, 7);
        assert_eq!(parsed[0].y1_bin, 25);

        assert_eq!(parsed[1].page, 8);
    }
}
