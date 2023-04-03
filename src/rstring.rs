use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct RString(pub Vec<String>);