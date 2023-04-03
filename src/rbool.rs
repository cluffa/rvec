use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct RBool(pub Vec<bool>);