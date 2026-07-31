use crate::configs::db::ConfigDB;
use crate::parsers::flows::layout_to_text_items::layout_to_text_items;
use crate::parsers::flows::pdf_to_text_items::pdf_to_text_items;
use crate::parsers::flows::text_items_to_statement_data::text_items_to_statement_data;
use crate::structs::{ProtoTransaction, StatementData};
use pdfsink_rs::PdfDocument;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct JsTransaction {
    date: i64,
    index: usize,
    description: String,
    amount: f64,
    balance: f64,
}

#[derive(Serialize)]
struct JsStatementData {
    key: String,
    account_number: String,
    start_date: i64,
    start_date_year: i32,
    opening_balance: f64,
    closing_balance: f64,
    transactions: Vec<JsTransaction>,
}

#[wasm_bindgen(js_name = Parser)]
pub struct WasmParser {
    db: ConfigDB,
    last_deprecation_warnings: Vec<String>,
}

#[wasm_bindgen(js_class = Parser)]
impl WasmParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            db: ConfigDB::new(),
            last_deprecation_warnings: Vec::new(),
        }
    }

    #[wasm_bindgen(js_name = getDeprecationWarnings)]
    pub fn get_deprecation_warnings(&self) -> Box<[JsValue]> {
        self.last_deprecation_warnings
            .iter()
            .cloned()
            .map(JsValue::from)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    #[wasm_bindgen(js_name = loadConfigFromJson)]
    pub fn load_config_from_json(&mut self, config_json: String) -> Result<(), JsValue> {
        self.last_deprecation_warnings.clear();
        let warnings = self
            .db
            .register_from_str_with_warnings(&config_json)
            .map_err(|e| JsValue::from_str(&e))?;
        self.last_deprecation_warnings = warnings;
        Ok(())
    }

    #[wasm_bindgen(js_name = load)]
    pub fn load(&mut self, config_json_path: String) -> Result<(), JsValue> {
        self.last_deprecation_warnings.clear();
        let content = std::fs::read_to_string(&config_json_path).map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to read config file at {}: {}",
                config_json_path, e
            ))
        })?;

        let warnings = self
            .db
            .register_from_str_with_warnings(&content)
            .map_err(|e| JsValue::from_str(&e))?;
        self.last_deprecation_warnings = warnings;
        Ok(())
    }

    #[wasm_bindgen(js_name = parse)]
    pub fn parse(&self, pdf_path: String) -> Result<JsValue, JsValue> {
        let doc = PdfDocument::open(&pdf_path).map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to open PDF document at {}: {}",
                pdf_path, e
            ))
        })?;
        self.parse_from_pdf_document(&doc)
    }

    #[wasm_bindgen(js_name = parseBytes)]
    pub fn parse_bytes(&self, _pdf_bytes: &[u8]) -> Result<JsValue, JsValue> {
        Err(JsValue::from_str(
            "Byte-based PDF parsing is not available yet in pdfsink-rs. Use parse(filePath) for now.",
        ))
    }

    #[wasm_bindgen(js_name = parseLayout)]
    pub fn parse_layout(&self, layout_path: String) -> Result<JsValue, JsValue> {
        let content = std::fs::read_to_string(&layout_path).map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to read layout file at {}: {}",
                layout_path, e
            ))
        })?;
        self.parse_layout_text(content)
    }

    #[wasm_bindgen(js_name = parseLayoutText)]
    pub fn parse_layout_text(&self, layout_text: String) -> Result<JsValue, JsValue> {
        let items = layout_to_text_items(&layout_text)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse layout text: {}", e)))?;
        let data = text_items_to_statement_data(&self.db, &items)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse statement data: {}", e)))?;
        statement_data_to_js(&data)
    }
}

impl WasmParser {
    fn parse_from_pdf_document(&self, doc: &PdfDocument) -> Result<JsValue, JsValue> {
        let items = pdf_to_text_items(doc)
            .map_err(|e| JsValue::from_str(&format!("Failed to extract text items: {}", e)))?;
        let data = text_items_to_statement_data(&self.db, &items)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse statement data: {}", e)))?;
        statement_data_to_js(&data)
    }
}

fn statement_data_to_js(data: &StatementData) -> Result<JsValue, JsValue> {
    let js_data = JsStatementData::try_from(data)?;
    to_value(&js_data).map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to serialize statement data to JavaScript value: {}",
            e
        ))
    })
}

impl TryFrom<&ProtoTransaction> for JsTransaction {
    type Error = JsValue;

    fn try_from(value: &ProtoTransaction) -> Result<Self, Self::Error> {
        Ok(Self {
            date: value.date.ok_or_else(|| {
                JsValue::from_str("Parsed transaction is missing required field: date")
            })?,
            index: value.index,
            description: value.description.clone(),
            amount: value.amount.ok_or_else(|| {
                JsValue::from_str("Parsed transaction is missing required field: amount")
            })?,
            balance: value.balance.ok_or_else(|| {
                JsValue::from_str("Parsed transaction is missing required field: balance")
            })?,
        })
    }
}

impl TryFrom<&StatementData> for JsStatementData {
    type Error = JsValue;

    fn try_from(value: &StatementData) -> Result<Self, Self::Error> {
        if !value.errors.is_empty() {
            return Err(JsValue::from_str(
                "Parsed statement data must be error-free before export to JavaScript",
            ));
        }

        let transactions = value
            .proto_transactions
            .iter()
            .map(JsTransaction::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            key: value.key.clone().ok_or_else(|| {
                JsValue::from_str("Parsed statement data is missing required field: key")
            })?,
            account_number: value.account_number.clone().ok_or_else(|| {
                JsValue::from_str("Parsed statement data is missing required field: account_number")
            })?,
            start_date: value.start_date.ok_or_else(|| {
                JsValue::from_str("Parsed statement data is missing required field: start_date")
            })?,
            start_date_year: value.start_date_year.ok_or_else(|| {
                JsValue::from_str(
                    "Parsed statement data is missing required field: start_date_year",
                )
            })?,
            opening_balance: value.opening_balance.ok_or_else(|| {
                JsValue::from_str(
                    "Parsed statement data is missing required field: opening_balance",
                )
            })?,
            closing_balance: value.closing_balance.ok_or_else(|| {
                JsValue::from_str(
                    "Parsed statement data is missing required field: closing_balance",
                )
            })?,
            transactions,
        })
    }
}

impl Default for WasmParser {
    fn default() -> Self {
        Self::new()
    }
}
