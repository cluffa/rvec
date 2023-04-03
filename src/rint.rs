use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct RInt(pub Vec<i64>);

#[pymethods]
impl RInt {
    #[staticmethod]
    pub fn new() -> Self {
        RInt(Vec::new())
    }
    
    // generic push that can take i64 or RInt, or Vec<i64> and python equivalents
    pub fn push(&mut self, x: &PyAny) -> PyResult<()> {
        if let Ok(x) = x.extract::<i64>() {
            self.0.push(x);
        } else if let Ok(x) = x.extract::<RInt>() {
            self.0.extend(x.0.clone());
        } else if let Ok(x) = x.extract::<Vec<i64>>() {
            self.0.extend(x);
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"));
        }
        Ok(())
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
    
    // indexing, can take i64 or usize, or python equivalents
    pub fn __getitem__(&self, idx: usize) -> PyResult<i64> {
        Ok(self.0[idx])
    }

    pub fn __setitem__(&mut self, idx: usize, val: i64) {
        self.0[idx] = val;
    }

    // element-wise operations work with RInt of same length or RInt of length 1
    pub fn __add__(&self, other: &RInt) -> PyResult<RInt> {
        if self.0.len() == other.0.len() {
            Ok(RInt(self.0.iter().zip(other.0.iter()).map(|(a, b)| a + b).collect()))
        } else if other.0.len() == 1 {
            Ok(RInt(self.0.iter().map(|a| a + other.0[0]).collect()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }

    pub fn __sub__(&self, other: &RInt) -> PyResult<RInt> {
        if self.0.len() == other.0.len() {
            Ok(RInt(self.0.iter().zip(other.0.iter()).map(|(a, b)| a - b).collect()))
        } else if other.0.len() == 1 {
            Ok(RInt(self.0.iter().map(|a| a - other.0[0]).collect()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }

    pub fn __mul__(&self, other: &RInt) -> PyResult<RInt> {
        if self.0.len() == other.0.len() {
            Ok(RInt(self.0.iter().zip(other.0.iter()).map(|(a, b)| a * b).collect()))
        } else if other.0.len() == 1 {
            Ok(RInt(self.0.iter().map(|a| a * other.0[0]).collect()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }

    pub fn __truediv__(&self, other: &RInt) -> PyResult<RFloat> {
        if self.0.len() == other.0.len() {
            Ok(RFloat(self.0.iter().zip(other.0.iter()).map(|(a, b)| *a as f64 / *b as f64).collect()))
        } else if other.0.len() == 1 {
            Ok(RFloat(self.0.iter().map(|a| *a as f64 / other.0[0] as f64).collect()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }

    pub fn __floordiv__(&self, other: &RInt) -> PyResult<RInt> {
        if self.0.len() == other.0.len() {
            Ok(RInt(self.0.iter().zip(other.0.iter()).map(|(a, b)| a / b).collect()))
        } else if other.0.len() == 1 {
            Ok(RInt(self.0.iter().map(|a| a / other.0[0]).collect()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }

    pub fn __mod__(&self, other: &RInt) -> PyResult<RInt> {
        if self.0.len() == other.0.len() {
            Ok(RInt(self.0.iter().zip(other.0.iter()).map(|(a, b)| a % b).collect()))
        } else if other.0.len() == 1 {
            Ok(RInt(self.0.iter().map(|a| a % other.0[0]).collect()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }

    // pub fn __pow__(&self, other: &RInt) -> PyResult<RInt> {
    //     if self.0.len() == other.0.len() {
    //         Ok(RInt(self.0.iter().zip(other.0.iter()).map(|(a, b)| a.pow(*b as u32)).collect()))
    //     } else if other.0.len() == 1 {
    //         Ok(RInt(self.0.iter().map(|a| a.pow(other.0[0] as u32)).collect()))
    //     } else {
    //         Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
    //     }
    // }

    pub fn __neg__(&self) -> RInt {
        RInt(self.0.iter().map(|a| -a).collect())
    }

    pub fn __pos__(&self) -> RInt {
        RInt(self.0.clone())
    }

    pub fn __abs__(&self) -> RInt {
        RInt(self.0.iter().map(|a| a.abs()).collect())
    }

    pub fn __invert__(&self) -> RInt {
        RInt(self.0.iter().map(|a| !a).collect())
    }

    pub fn __lshift__(&self, other: &RInt) -> PyResult<RInt> {
        if self.0.len() == other.0.len() {
            Ok(RInt(self.0.iter().zip(other.0.iter()).map(|(a, b)| a << b).collect()))
        } else if other.0.len() == 1 {
            Ok(RInt(self.0.iter().map(|a| a << other.0[0]).collect()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }

    pub fn __rshift__(&self, other: &RInt) -> PyResult<RInt> {
        if self.0.len() == other.0.len() {
            Ok(RInt(self.0.iter().zip(other.0.iter()).map(|(a, b)| a >> b).collect()))
        } else if other.0.len() == 1 {
            Ok(RInt(self.0.iter().map(|a| a >> other.0[0]).collect()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }

    pub fn __and__(&self, other: &RInt) -> PyResult<RInt> {
        if self.0.len() == other.0.len() {
            Ok(RInt(self.0.iter().zip(other.0.iter()).map(|(a, b)| a & b).collect()))
        } else if other.0.len() == 1 {
            Ok(RInt(self.0.iter().map(|a| a & other.0[0]).collect()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }

    pub fn __xor__(&self, other: &RInt) -> PyResult<RInt> {
        if self.0.len() == other.0.len() {
            Ok(RInt(self.0.iter().zip(other.0.iter()).map(|(a, b)| a ^ b).collect()))
        } else if other.0.len() == 1 {
            Ok(RInt(self.0.iter().map(|a| a ^ other.0[0]).collect()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }

    pub fn __or__(&self, other: &RInt) -> PyResult<RInt> {
        if self.0.len() == other.0.len() {
            Ok(RInt(self.0.iter().zip(other.0.iter()).map(|(a, b)| a | b).collect()))
        } else if other.0.len() == 1 {
            Ok(RInt(self.0.iter().map(|a| a | other.0[0]).collect()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }

    pub fn __bool__(&self) -> bool {
        self.0.iter().any(|a| *a != 0)
    }

    pub fn __float__(&self) -> PyResult<RFloat> {
        if self.0.len() == 1 {
            Ok(RFloat(vec![self.0[0] as f64]))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }
}