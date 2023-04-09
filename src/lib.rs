use pyo3::prelude::*;

mod vec_data;
mod string_methods;
mod vec_operations;

/// default percision
const DEFAULT_TO_64: bool = false;

use vec_data::{RVecData, from_py, BaseRVecData};
use string_methods::VecStringMethods;

#[pyclass]
#[derive(Clone, Debug)]
struct RVec {
    data: RVecData,
}

#[pymethods]
impl RVec {
    #[new]
    pub fn new(data: &PyAny) -> PyResult<Self> {
        Ok(RVec { data: from_py(data)? })
    }

    #[pyo3(text_signature = "($self, /)")]
    pub fn str(&self) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str() })
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.data))
    }

    pub fn __len__(&self) -> PyResult<usize> {
        Ok(self.data.len())
    }

    pub fn __add__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(RVec { data: self.data.clone() + other.data.clone() })
        } else {
            Ok(RVec { data: self.data.clone() + from_py(&other)? })
        }
    }

    pub fn __radd__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(RVec { data: other.data.clone() + self.data.clone() })
        } else {
            Ok(RVec { data: from_py(&other)? + self.data.clone() })
        }
    }

    pub fn __sub__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(RVec { data: self.data.clone() - other.data.clone() })
        } else {
            Ok(RVec { data: self.data.clone() - from_py(&other)? })
        }
    }

    pub fn __rsub__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(RVec { data: other.data.clone() - self.data.clone() })
        } else {
            Ok(RVec { data: from_py(&other)? - self.data.clone() })
        }
    }

    pub fn __mul__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(RVec { data: self.data.clone() * other.data.clone() })
        } else {
            Ok(RVec { data: self.data.clone() * from_py(&other)? })
        }
    }

    pub fn __rmul__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(RVec { data: other.data.clone() * self.data.clone() })
        } else {
            Ok(RVec { data: from_py(&other)? * self.data.clone() })
        }
    }

    pub fn __truediv__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(RVec { data: self.data.clone() / other.data.clone() })
        } else {
            Ok(RVec { data: self.data.clone() / from_py(&other)? })
        }
    }

    pub fn __rtruediv__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(RVec { data: other.data.clone() / self.data.clone() })
        } else {
            Ok(RVec { data: from_py(&other)? / self.data.clone() })
        }
    }

    pub fn __neg__(&self) -> PyResult<Self> {
        Ok(RVec { data: -self.data.clone() })
    }
    
    pub fn __getitem__(&self, index: &PyAny) -> PyResult<f64> {
        panic!("Not implemented {}", index) // TODO
    }

    pub fn __setitem__(&mut self, index: &PyAny, value: &PyAny) -> PyResult<()> {
        panic!("Not implemented {} {}", index, value) // TODO
    }

    pub fn str_capitalize(&self) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().capitalize() })
    }

    pub fn str_center(&self, width: usize, fill_char: char) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().center(width, fill_char) })
    }

    pub fn str_count(&self, sub: &str) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().count(sub) })
    }

    pub fn str_endswith(&self, suffix: &str) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().endswith(suffix) })
    }

    pub fn str_startswith(&self, prefix: &str) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().startswith(prefix) })
    }

    pub fn str_find(&self, sub: &str) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().find(sub) })
    }

    pub fn str_lower(&self) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().lower() })
    }

    pub fn str_upper(&self) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().upper() })
    }

    pub fn str_replace(&self, old: &str, new: &str) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().replace(old, new) })
    }

    pub fn str_split(&self, sep: &str) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().split(sep) })
    }

    pub fn str_strip(&self) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().strip() })
    }

    pub fn str_lstrip(&self) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().lstrip() })
    }

    pub fn str_rstrip(&self) -> PyResult<Self> {
        Ok(RVec { data: self.data.as_str().rstrip() })
    }
}

#[pymodule]
fn rvec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RVec>()?;
    Ok(())
}