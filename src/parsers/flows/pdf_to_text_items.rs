use crate::structs::TextItem;
use pdfsink_rs::{PdfDocument, TextOptions};

struct Bins {
    /// Array of bools representing the y-coordinate bins.
    y_bins: Vec<bool>,
}

impl Bins {
    pub fn new() -> Self {
        // Array of 1001 false bins, representing y-coordinates from 0-1000.
        Self {
            y_bins: vec![false; 1001], // All bins unoccupied
        }
    }

    /// Get the closest true bin for the given y-coordinate within given tolerance,
    /// or return None
    pub fn get_closest_bin(&mut self, y1: i32, tolerance: i32) -> Option<i32> {
        let y1_usize = y1.max(0) as usize; // Ensure y1 is non-negative
        let len = self.y_bins.len();

        // Directly return bin if it already exists
        if self.y_bins[y1_usize] {
            return Some(y1);
        }

        // Check surrounding within tolerance
        for offset in -(tolerance)..=tolerance {
            let check_y = y1_usize as i32 + offset;
            if check_y >= 0 && (check_y as usize) < len && self.y_bins[check_y as usize] {
                return Some(check_y); // Return the closest existing bin
            }
        }

        // If no existing bin is found
        None
    }

    /// Set bin for a given y-coordinate to true, extending the bins vector if necessary.
    pub fn set_bin(&mut self, y1: i32) {
        let y1_usize = y1.max(0) as usize; // Ensure y1 is non-negative
        if y1_usize >= self.y_bins.len() {
            self.y_bins.resize(y1_usize + 500, false);
        }
        self.y_bins[y1_usize] = true
    }
}

/// Calculate average character width for a TextItem
fn average_char_width(item: &TextItem) -> f32 {
    let width = item.x2 - item.x1;
    let num_chars = item.text.len() as i32;
    if num_chars == 0 {
        0.0
    } else {
        width as f32 / num_chars as f32
    }
}

/// Extract a PDF document into a canonically ordered vector of TextItems
///
/// Extracts the PDF page by page and word by word. Merges words belonging to same
/// block (TextItem), then enforces that TextItems are sorted by page, y-position
/// and x-position. A dynamic y-coordinate binning system is used to tolerate small
/// variations in line alignment.
pub fn pdf_to_text_items(pdf_doc: &PdfDocument) -> Result<Vec<TextItem>, String> {
    let mut text_items = Vec::new();
    let mut y1_bins = Bins::new();
    let x_tol = 2.0; // For characters to merge into same word
    let y_tol = 3.0; // For characters to merge into same word, or TextItems into same line
    let x_gap = 1; // Character gap for words to be merged into same TextItem

    // Configure text extraction options with x_tolerance=2, y_tolerance=3
    // x_tolerance reduced from default due to many encounters of improper word merging
    let text_options = TextOptions::default().with_tolerances(x_tol, y_tol);

    for (page_index, page) in pdf_doc.pages().iter().enumerate() {
        let mut curr_y1_bin = 0;

        for word in page.extract_words_with_options(&text_options, false) {
            // Ignore non-horizontal or non-upright text, which never provide useful
            // information and often interferes with extraction.
            if !word.direction.is_horizontal() || !word.upright {
                continue;
            }

            let text = word.text.trim();
            if text.is_empty() {
                continue;
            }

            let mut this_text_item = TextItem::new(
                text.to_string(),
                (word.x0.round() as i32).max(0),
                (word.bottom.round() as i32).max(0),
                (word.x1.round() as i32).max(0),
                (word.top.round() as i32).max(0),
                page_index as i32,
            );

            // Assign item to a y1_bin for later sorting.
            // Item is on the same line as last item
            if (this_text_item.y1 - curr_y1_bin).abs() <= y_tol as i32 {
                this_text_item.y1_bin = curr_y1_bin;
            }
            // Item is on the next line
            else if this_text_item.y1 > curr_y1_bin + y_tol as i32 {
                y1_bins.set_bin(this_text_item.y1);
                curr_y1_bin = this_text_item.y1
            }
            // Out-of-order case where item belongs to a previous line. Assign to
            // closest bin with an extended tolerance to account for greater deviations
            // in y-alignment for such cases. If no bin is found, create a new one.
            else {
                if let Some(closest_bin) =
                    y1_bins.get_closest_bin(this_text_item.y1, y_tol as i32 * 2)
                {
                    this_text_item.y1_bin = closest_bin;
                } else {
                    y1_bins.set_bin(this_text_item.y1);
                    curr_y1_bin = this_text_item.y1;
                }
            }

            // Merge with last item if on same line and close enough
            let last_item_opt = text_items.last_mut();
            if let Some(last_item) = last_item_opt {
                // Calculate average character width from both items
                let char_width =
                    (average_char_width(last_item) + average_char_width(&this_text_item)) / 2.0;
                let max_distance = x_gap * char_width as i32;

                // Check if items are on the same line and close enough
                let top_diff = (this_text_item.y1 - last_item.y1).abs();
                let bottom_diff = (this_text_item.y2 - last_item.y2).abs();
                let horizontal_gap = this_text_item.x1 - last_item.x2;

                if top_diff <= 3
                    && bottom_diff <= 3
                    && horizontal_gap >= 0
                    && horizontal_gap <= max_distance
                {
                    last_item.merge(&this_text_item);
                    continue;
                }
            }

            // If not merged, push the current TextItem to the list
            text_items.push(this_text_item);
        }
    }

    // Sort items by page, y1_bin then x1
    text_items.sort_by(|a, b| {
        a.page
            .cmp(&b.page)
            .then(a.y1_bin.cmp(&b.y1_bin))
            .then(a.x1.cmp(&b.x1))
    });

    Ok(text_items)
}
