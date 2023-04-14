use pyo3::prelude::*;
use crate::{Idef, Fdef};

// A vector of data
#[derive(Debug, Clone)]
pub enum RVecData {
    Int(Vec<Idef>),
    Float(Vec<Fdef>),
    Str(Vec<String>),
    Bool(Vec<bool>),
}

pub trait BaseRVecData {
    /// Returns the length of the vector
    fn len(&self) -> usize;
    /// Returns true if the vector is a scalar (length == 1)
    fn is_scalar(&self) -> bool;
    /// Returns the type of the vector (int, float, str, bool)
    fn element_type(&self) -> &'static str;
    /// Converts the vector to a vector of strings
    fn as_str(&self) -> RVecData;
    /// Converts the vector to a vector of floats
    fn as_float(&self) -> RVecData;
    /// Converts the vector to a vector of ints
    fn as_int(&self) -> RVecData;
    /// Converts the vector to a vector of python objects
    fn to_list(&self) -> PyResult<Vec<PyObject>>;
}

impl BaseRVecData for RVecData {
    fn len(&self) -> usize {
        match self {
            RVecData::Int(a) => a.len(),
            RVecData::Float(a) => a.len(),
            RVecData::Str(a) => a.len(),
            RVecData::Bool(a) => a.len(),
        }
    }

    fn is_scalar(&self) -> bool {
        self.len() == 1
    }

    fn element_type(&self) -> &'static str {
        match self {
            RVecData::Int(_) => "int",
            RVecData::Float(_) => "float",
            RVecData::Str(_) => "str",
            RVecData::Bool(_) => "bool",
        }
    }

    fn as_str(&self) -> RVecData {
        match self {
            RVecData::Int(a) => RVecData::Str(a.iter().map(|x| x.to_string()).collect()),
            RVecData::Float(a) => RVecData::Str(a.iter().map(|x| x.to_string()).collect()),
            RVecData::Str(a) => RVecData::Str(a.clone()),
            RVecData::Bool(a) => RVecData::Str(a.iter().map(|x| x.to_string()).collect()),
        }
    }

    fn as_float(&self) -> RVecData {
        match self {
            RVecData::Int(a) => RVecData::Float(a.iter().map(|x| *x as Fdef).collect()),
            RVecData::Float(a) => RVecData::Float(a.clone()),
            _ => panic!("Cannot convert to float"),
        }
    }

    fn as_int(&self) -> RVecData {
        match self {
            RVecData::Int(a) => RVecData::Int(a.clone()),
            RVecData::Float(a) => RVecData::Int(a.iter().map(|x| *x as Idef).collect()),
            RVecData::Bool(a) => RVecData::Int(a.iter().map(|x| *x as Idef).collect()),
            _ => panic!("Cannot convert to int"),
        }
    }

    fn to_list(&self) -> PyResult<Vec<PyObject>> {
        Python::with_gil(|py| {
            match self {
                RVecData::Int(a) => Ok(a.iter().map(|x| x.to_object(py)).collect()),
                RVecData::Float(a) => Ok(a.iter().map(|x| x.to_object(py)).collect()),
                RVecData::Str(a) => Ok(a.iter().map(|x| x.to_object(py)).collect()),
                RVecData::Bool(a) => Ok(a.iter().map(|x| x.to_object(py)).collect()),
            }
        })
    }
}

/// Converts a Python object to RVecData
pub fn from_py(obj: &PyAny) -> PyResult<RVecData> {
    if let Ok(_) = obj.extract::<Vec<bool>>() {
        Ok(RVecData::Bool(obj.extract()?))
    } else if let Ok(_) = obj.extract::<Vec<Idef>>() {
        Ok(RVecData::Int(obj.extract()?))
    } else if let Ok(_) = obj.extract::<Vec<Fdef>>() {
        Ok(RVecData::Float(obj.extract()?))
    } else if let Ok(_) = obj.extract::<Vec<String>>() {
        Ok(RVecData::Str(obj.extract()?))
    } else if let Ok(_) = obj.extract::<bool>() {
        Ok(RVecData::Bool(vec![obj.extract()?]))
    } else if let Ok(_) = obj.extract::<Idef>() {
        Ok(RVecData::Int(vec![obj.extract()?]))
    } else if let Ok(_) = obj.extract::<Fdef>() {
        Ok(RVecData::Float(vec![obj.extract()?]))
    } else if let Ok(_) = obj.extract::<String>() {
        Ok(RVecData::Str(vec![obj.extract()?]))
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
    }
}
