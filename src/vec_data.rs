use pyo3::prelude::*;
const DEFAULT_TO_64: bool = crate::DEFAULT_TO_64;

// A vector of data
#[derive(Debug, Clone)]
pub enum RVecData {
    I32(Vec<i32>),
    I64(Vec<i64>),
    F32(Vec<f32>),
    F64(Vec<f64>),
    Str(Vec<String>),
    Bool(Vec<bool>),
}

pub trait BaseRVecData {
    /// Returns the length of the vector
    fn len(&self) -> usize;
    /// Returns true if the vector is a scalar (length == 1)
    fn is_scalar(&self) -> bool;
    /// Returns the type of the vector (i32, i64, f32, f64, str, bool)
    fn element_type(&self) -> &'static str;
    /// Converts the vector to a vector of strings
    fn as_str(&self) -> RVecData;
}

impl BaseRVecData for RVecData {
    fn len(&self) -> usize {
        match self {
            RVecData::I32(a) => a.len(),
            RVecData::I64(a) => a.len(),
            RVecData::F32(a) => a.len(),
            RVecData::F64(a) => a.len(),
            RVecData::Str(a) => a.len(),
            RVecData::Bool(a) => a.len(),
        }
    }

    fn is_scalar(&self) -> bool {
        self.len() == 1
    }

    fn element_type(&self) -> &'static str {
        match self {
            RVecData::I32(_) => "i32",
            RVecData::I64(_) => "i64",
            RVecData::F32(_) => "f32",
            RVecData::F64(_) => "f64",
            RVecData::Str(_) => "str",
            RVecData::Bool(_) => "bool",
        }
    }

    fn as_str(&self) -> RVecData {
        match self {
            RVecData::I32(a) => RVecData::Str(a.iter().map(|x| x.to_string()).collect()),
            RVecData::I64(a) => RVecData::Str(a.iter().map(|x| x.to_string()).collect()),
            RVecData::F32(a) => RVecData::Str(a.iter().map(|x| x.to_string()).collect()),
            RVecData::F64(a) => RVecData::Str(a.iter().map(|x| x.to_string()).collect()),
            RVecData::Str(a) => RVecData::Str(a.clone()),
            RVecData::Bool(a) => RVecData::Str(a.iter().map(|x| x.to_string()).collect()),
        }
    }
}

/// Converts a Python object to RVecData
pub fn from_py(obj: &PyAny) -> PyResult<RVecData> {
    if DEFAULT_TO_64 {
        if let Ok(_) = obj.extract::<Vec<i64>>() {
            Ok(RVecData::I64(obj.extract()?))
        } else if let Ok(_) = obj.extract::<Vec<f64>>() {
            Ok(RVecData::F64(obj.extract()?))
        } else if let Ok(_) = obj.extract::<Vec<String>>() {
            Ok(RVecData::Str(obj.extract()?))
        } else if let Ok(_) = obj.extract::<Vec<bool>>() {
            Ok(RVecData::Bool(obj.extract()?))
        } else if let Ok(_) = obj.extract::<i64>() {
            Ok(RVecData::I64(vec![obj.extract()?]))
        } else if let Ok(_) = obj.extract::<f64>() {
            Ok(RVecData::F64(vec![obj.extract()?]))
        } else if let Ok(_) = obj.extract::<String>() {
            Ok(RVecData::Str(vec![obj.extract()?]))
        } else if let Ok(_) = obj.extract::<bool>() {
            Ok(RVecData::Bool(vec![obj.extract()?]))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    } else {
        if let Ok(_) = obj.extract::<Vec<i32>>() {
            Ok(RVecData::I32(obj.extract()?))
        } else if let Ok(_) = obj.extract::<Vec<f32>>() {
            Ok(RVecData::F32(obj.extract()?))
        } else if let Ok(_) = obj.extract::<Vec<String>>() {
            Ok(RVecData::Str(obj.extract()?))
        } else if let Ok(_) = obj.extract::<Vec<bool>>() {
            Ok(RVecData::Bool(obj.extract()?))
        } else if let Ok(_) = obj.extract::<i32>() {
            Ok(RVecData::I32(vec![obj.extract()?]))
        } else if let Ok(_) = obj.extract::<f32>() {
            Ok(RVecData::F32(vec![obj.extract()?]))
        } else if let Ok(_) = obj.extract::<String>() {
            Ok(RVecData::Str(vec![obj.extract()?]))
        } else if let Ok(_) = obj.extract::<bool>() {
            Ok(RVecData::Bool(vec![obj.extract()?]))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
        }
    }
}
