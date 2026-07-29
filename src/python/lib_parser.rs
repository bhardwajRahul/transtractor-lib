use crate::configs::db::ConfigDB;
use crate::parsers::flows::layout_to_text_items::layout_to_text_items;
use crate::parsers::flows::pdf_to_text_items::pdf_to_text_items;
use crate::parsers::flows::text_items_to_debug::text_items_to_debug;
use crate::parsers::flows::text_items_to_layout::text_items_to_layout;
use crate::parsers::flows::text_items_to_statement_data::text_items_to_statement_data;
use crate::python::exceptions::{ConfigLoadError, ParseError};
use crate::python::utils;
use crate::structs::Spec;
use pdfsink_rs::PdfDocument;
use pyo3::prelude::*;

#[pyclass]
#[derive(Default)]
pub struct LibParser {
    db: ConfigDB,
    /// Last deprecation warnings from config loading
    last_deprecation_warnings: Vec<String>,
}

#[pymethods]
impl LibParser {
    /// Create a new Parser instance
    #[new]
    pub fn new() -> Self {
        Self {
            db: ConfigDB::new(),
            last_deprecation_warnings: Vec::new(),
        }
    }

    /// Register JSON configuration string into the parser database
    pub fn register_config_from_json_str(&mut self, py_json_str: &str) -> PyResult<()> {
        // Clear previous warnings
        self.last_deprecation_warnings.clear();

        // Register and get deprecation warnings
        match self.db.register_from_str_with_warnings(py_json_str) {
            Ok(warnings) => {
                self.last_deprecation_warnings = warnings;
                Ok(())
            }
            Err(e) => Err(ConfigLoadError::new_err(e)),
        }
    }

    /// Get deprecation warnings from the last loaded configuration
    pub fn get_deprecation_warnings(&self) -> Vec<String> {
        self.last_deprecation_warnings.clone()
    }

    /// Import JSON configuration file into the parser database and update the StatementTyper.
    pub fn register_config_from_file(&mut self, py_file_path: &str) -> PyResult<()> {
        match self.db.register_from_file(py_file_path) {
            Ok(_) => {}
            Err(e) => return Err(ConfigLoadError::new_err(e)),
        }
        Ok(())
    }

    /// Process a layout string and return statement data as a Python object of type
    /// StatementData.
    pub fn layout_py_str_py_statement_data(
        &self,
        py_layout_str: &Bound<'_, PyAny>,
    ) -> PyResult<Py<PyAny>> {
        let rust_layout_str = py_layout_str.extract::<String>()?;
        let text_items = layout_to_text_items(&rust_layout_str).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to convert layout string to text items: {}",
                e
            ))
        })?;
        let data =
            text_items_to_statement_data(&self.db, &text_items).map_err(ParseError::new_err)?;
        utils::rust_statement_data_to_py_statement_data(&data)
    }

    pub fn py_pdf_path_to_layout_py_str(&self, py_pdf_path: &Bound<'_, PyAny>) -> PyResult<String> {
        let rust_pdf_path = py_pdf_path.extract::<String>()?;
        let pdf_document = PdfDocument::open(&rust_pdf_path).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to open PDF document: {}", e))
        })?;
        let text_items = pdf_to_text_items(&pdf_document).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to convert PDF to text items: {}",
                e
            ))
        })?;
        match text_items_to_layout(&text_items) {
            Ok(layout_str) => Ok(layout_str),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e)),
        }
    }

    /// Process a PDF file path from Python caller and return debug information as a string.
    pub fn py_pdf_path_to_debug_py_str(&self, py_pdf_path: &Bound<'_, PyAny>) -> PyResult<String> {
        let rust_pdf_path = py_pdf_path.extract::<String>()?;
        let pdf_document = PdfDocument::open(&rust_pdf_path).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to open PDF document: {}", e))
        })?;
        let text_items = pdf_to_text_items(&pdf_document).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to convert PDF to text items: {}",
                e
            ))
        })?;
        let debug_str = text_items_to_debug(&self.db, &text_items).map_err(ParseError::new_err)?;
        Ok(debug_str)
    }

    /// Process a layout string from Python caller and return debug information as a string.
    pub fn layout_py_str_to_debug_py_str(
        &self,
        py_layout_str: &Bound<'_, PyAny>,
    ) -> PyResult<String> {
        let rust_layout_str = py_layout_str.extract::<String>()?;
        let text_items = layout_to_text_items(&rust_layout_str).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to convert layout string to text items: {}",
                e
            ))
        })?;
        let debug_str = text_items_to_debug(&self.db, &text_items).map_err(ParseError::new_err)?;
        Ok(debug_str)
    }

    /// Process a PDF file path from Python caller and return a Python StatementData object.
    pub fn py_pdf_path_to_py_statement_data(
        &self,
        py_pdf_path: &Bound<'_, PyAny>,
    ) -> PyResult<Py<PyAny>> {
        let rust_pdf_path = py_pdf_path.extract::<String>()?;
        let pdf_document = PdfDocument::open(&rust_pdf_path).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to open PDF document: {}", e))
        })?;
        let text_items = pdf_to_text_items(&pdf_document).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to convert PDF to text items: {}",
                e
            ))
        })?;
        let data =
            text_items_to_statement_data(&self.db, &text_items).map_err(ParseError::new_err)?;
        utils::rust_statement_data_to_py_statement_data(&data)
    }

    /// Process a PDF file path from Python caller and return a JSON spec string.
    pub fn py_pdf_path_to_spec_py_str(&self, py_pdf_path: &Bound<'_, PyAny>) -> PyResult<String> {
        let rust_pdf_path = py_pdf_path.extract::<String>()?;
        let pdf_document = PdfDocument::open(&rust_pdf_path).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to open PDF document: {}", e))
        })?;
        let text_items = pdf_to_text_items(&pdf_document).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to convert PDF to text items: {}",
                e
            ))
        })?;
        let spec = Spec::new(
            text_items_to_statement_data(&self.db, &text_items).map_err(ParseError::new_err)?,
            text_items,
        );
        spec.to_json().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to convert Spec to JSON string: {}",
                e
            ))
        })
    }
}
