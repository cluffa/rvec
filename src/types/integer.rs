use pyo3::prelude::*;

#[allow(unused_imports)]
use super::{float::FloatVec, string::StrVec, bool::BoolVec};

#[pyclass]
#[derive(Clone)]
pub struct IntVec {
    pub data: Vec<i64>,
}

#[pymethods]
impl IntVec {
    #[new]
    pub fn new(data: Vec<i64>) -> Self {
        Self { data }
    }

    #[pyo3(text_signature = "($self)")]
    fn len(&self) -> PyResult<usize> {
        Ok(self.data.len())
    }

    #[getter]
    pub fn get_data(&self) -> Vec<i64> {
        self.data.clone()
    }

    #[setter]
    pub fn set_data(&mut self, data: Vec<i64>) {
        self.data = data;
    }

    #[staticmethod]
    pub fn zeros(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn to_list(&self) -> PyResult<Vec<i64>> {
        Ok(self.data.clone())
    }

    pub fn to_float(&self) -> PyResult<FloatVec> {
        Ok(FloatVec {
            data: self.data.iter().map(|x| *x as f64).collect(),
        })
    }
}