use pyo3::prelude::*;

mod rvecdata;
use rvecdata::{RVecData, from_py, BaseRVecData};

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
        panic!("Not implemented")
    }

    pub fn __setitem__(&mut self, index: &PyAny, value: &PyAny) -> PyResult<()> {
        panic!("Not implemented")
    }
}

#[pymodule]
fn rvec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RVec>()?;
    Ok(())
}