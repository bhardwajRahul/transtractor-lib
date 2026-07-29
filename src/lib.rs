pub mod checkers;
pub mod configs;
pub mod fixers;
pub mod formats;
pub mod parsers;
pub mod python;
pub mod structs;

use crate::python::exceptions::{ConfigLoadError, ParseError};
use crate::python::lib_parser::LibParser;
use pyo3::prelude::*;

/// Python module definition
#[pymodule]
fn transtractor(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LibParser>()?;
    m.add("ConfigLoadError", m.py().get_type::<ConfigLoadError>())?;
    m.add("ParseError", m.py().get_type::<ParseError>())?;
    Ok(())
}
