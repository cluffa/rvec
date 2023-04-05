use pyo3::prelude::*;

#[allow(unused_imports)]
use super::{float::FloatVec, integer::IntVec, bool::BoolVec};

#[pyclass]
#[derive(Clone)]
pub struct StrVec {
    pub data: Vec<String>,
}

#[pymethods]
impl StrVec {
    #[new]
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }
}