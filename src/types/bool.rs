use pyo3::{prelude::*, exceptions::PyTypeError};

#[allow(unused_imports)]
use super::{float::FloatVec, integer::IntVec, string::StrVec};

#[pyclass]
#[derive(Clone)]
pub struct BoolVec {
    pub data: Vec<bool>,
}

#[pymethods]
impl BoolVec {
    #[new]
    pub fn new(data: Vec<bool>) -> Self {
        BoolVec { data }
    }

    #[getter]
    fn get_data(&self) -> Vec<bool> {
        self.data.clone()
    }

    #[setter]
    fn set_data(&mut self, data: Vec<bool>) {
        self.data = data;
    }

    #[pyo3(text_signature = "($self)")]
    fn len(&self) -> usize {
        self.data.len()
    }

    #[pyo3(text_signature = "($self)")]
    fn count(&self) -> usize {
        self.data.iter().filter(|x| **x).count()
    }

    #[pyo3(text_signature = "($self)")]
    fn all(&self) -> bool {
        self.data.iter().all(|x| *x)
    }

    #[pyo3(text_signature = "($self)")]
    fn any(&self) -> bool {
        self.data.iter().any(|x| *x)
    }

    // ############### Python methods ############### //
    // needs to be implemented for PyAny for elementwise and vectorwise operations
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Vector({:?})", self.data))
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("Vector({:?})", self.data))
    }
    
    pub fn __eq__(&self, other: &PyAny) -> PyResult<BoolVec> {
        if let Ok(other) = other.extract::<bool>() {
            Ok(BoolVec {
                data: self.data.iter().map(|x| *x == other).collect(),
            })
        } else if let Ok(other) = other.extract::<BoolVec>() {
            Ok(BoolVec {
                data: self.data.iter().zip(other.data.iter()).map(|(x, y)| *x == *y).collect(),
            })
        } else {
            Err(PyTypeError::new_err("Cannot compare BoolVec with other type"))
        }
    }

    pub fn __ne__(&self, other: &PyAny) -> PyResult<BoolVec> {
        if let Ok(other) = other.extract::<bool>() {
            Ok(BoolVec {
                data: self.data.iter().map(|x| *x != other).collect(),
            })
        } else if let Ok(other) = other.extract::<BoolVec>() {
            Ok(BoolVec {
                data: self.data.iter().zip(other.data.iter()).map(|(x, y)| *x != *y).collect(),
            })
        } else {
            Err(PyTypeError::new_err("Cannot compare BoolVec with other type"))
        }
    }

    pub fn __and__(&self, other: &PyAny) -> PyResult<BoolVec> {
        if let Ok(other) = other.extract::<bool>() {
            Ok(BoolVec {
                data: self.data.iter().map(|x| *x && other).collect(),
            })
        } else if let Ok(other) = other.extract::<BoolVec>() {
            Ok(BoolVec {
                data: self.data.iter().zip(other.data.iter()).map(|(x, y)| *x && *y).collect(),
            })
        } else {
            Err(PyTypeError::new_err("Cannot compare BoolVec with other type"))
        }
    }

    pub fn __or__(&self, other: &PyAny) -> PyResult<BoolVec> {
        if let Ok(other) = other.extract::<bool>() {
            Ok(BoolVec {
                data: self.data.iter().map(|x| *x || other).collect(),
            })
        } else if let Ok(other) = other.extract::<BoolVec>() {
            Ok(BoolVec {
                data: self.data.iter().zip(other.data.iter()).map(|(x, y)| *x || *y).collect(),
            })
        } else {
            Err(PyTypeError::new_err("Cannot compare BoolVec with other type"))
        }
    }

    pub fn __xor__(&self, other: &PyAny) -> PyResult<BoolVec> {
        if let Ok(other) = other.extract::<bool>() {
            Ok(BoolVec {
                data: self.data.iter().map(|x| *x ^ other).collect(),
            })
        } else if let Ok(other) = other.extract::<BoolVec>() {
            Ok(BoolVec {
                data: self.data.iter().zip(other.data.iter()).map(|(x, y)| *x ^ *y).collect(),
            })
        } else {
            Err(PyTypeError::new_err("Cannot compare BoolVec with other type"))
        }
    }

    pub fn __invert__(&self) -> BoolVec {
        BoolVec {
            data: self.data.iter().map(|x| !x).collect(),
        }
    }

    // ############### LOGIC ############### //
    #[pyo3(text_signature = "($self, other)")]
    fn and(&self, other: &BoolVec) -> BoolVec {
        BoolVec {
            data: self.data.iter().zip(other.data.iter()).map(|(x, y)| *x && *y).collect(),
        }
    }

    #[pyo3(text_signature = "($self, other)")]
    fn or(&self, other: &BoolVec) -> BoolVec {
        BoolVec {
            data: self.data.iter().zip(other.data.iter()).map(|(x, y)| *x || *y).collect(),
        }
    }

    #[pyo3(text_signature = "($self, other)")]
    fn xor(&self, other: &BoolVec) -> BoolVec {
        BoolVec {
            data: self.data.iter().zip(other.data.iter()).map(|(x, y)| *x ^ *y).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    fn not(&self) -> BoolVec {
        BoolVec {
            data: self.data.iter().map(|x| !x).collect(),
        }
    }

    // ############### COMPARISON ############### //
    pub fn equals(&self, other: &BoolVec) -> BoolVec {
        BoolVec {
            data: self.data.iter().zip(other.data.iter()).map(|(x, y)| x == y).collect(),
        }
    }

    // element-wise
    pub fn equals_element(&self, other: &BoolVec) -> BoolVec {
        BoolVec {
            data: self.data.iter().zip(other.data.iter()).map(|(x, y)| x == y).collect(),
        }
    }

}