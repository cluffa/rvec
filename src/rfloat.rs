use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct RFloat(pub Vec<f64>);