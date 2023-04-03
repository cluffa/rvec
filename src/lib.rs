use pyo3::prelude::*;
use pyo3::types::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct RInt(pub Vec<i64>);

#[pymethods]
impl RInt {
    #[staticmethod]
    pub fn new() -> Self {
        RInt(Vec::new())
    }
    
    pub fn push(&mut self, x: i64) {
        self.0.push(x);
    }

    pub fn push_list(&mut self, x: Vec<i64>) {
        self.0.extend(x);
    }

    pub fn concat(&mut self, x: &RInt) {
        self.0.extend(x.0.clone());
    }

    pub fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    pub fn __len__(&self) -> usize {
        self.0.len()
    }

    pub fn __getitem__(&self, idx: usize) -> i64 {
        self.0[idx]
    }

    pub fn __setitem__(&mut self, idx: usize, val: i64) {
        self.0[idx] = val;
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn rvec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<RInt>()?;
    Ok(())
}