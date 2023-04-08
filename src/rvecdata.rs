use std::ops::{Add, Div, Mul, Sub, Neg};
use pyo3::prelude::*;

/// A value that can be stored in a vector
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
    fn len(&self) -> usize;
    fn is_scalar(&self) -> bool;
    fn element_type(&self) -> &'static str;
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

pub fn from_py(obj: &PyAny) -> PyResult<RVecData> {
    if let Ok(_) = obj.extract::<Vec<i32>>() {
        Ok(RVecData::I32(obj.extract()?))
    } else if let Ok(_) = obj.extract::<Vec<i64>>() {
        Ok(RVecData::I64(obj.extract()?))
    } else if let Ok(_) = obj.extract::<Vec<f32>>() {
        Ok(RVecData::F32(obj.extract()?))
    } else if let Ok(_) = obj.extract::<Vec<f64>>() {
        Ok(RVecData::F64(obj.extract()?))
    } else if let Ok(_) = obj.extract::<Vec<String>>() {
        Ok(RVecData::Str(obj.extract()?))
    } else if let Ok(_) = obj.extract::<Vec<bool>>() {
        Ok(RVecData::Bool(obj.extract()?))
    } else if let Ok(_) = obj.extract::<i32>() {
        Ok(RVecData::I32(vec![obj.extract()?]))
    } else if let Ok(_) = obj.extract::<i64>() {
        Ok(RVecData::I64(vec![obj.extract()?]))
    } else if let Ok(_) = obj.extract::<f32>() {
        Ok(RVecData::F32(vec![obj.extract()?]))
    } else if let Ok(_) = obj.extract::<f64>() {
        Ok(RVecData::F64(vec![obj.extract()?]))
    } else if let Ok(_) = obj.extract::<String>() {
        Ok(RVecData::Str(vec![obj.extract()?]))
    } else if let Ok(_) = obj.extract::<bool>() {
        Ok(RVecData::Bool(vec![obj.extract()?]))
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid type"))
    }
}

// uses python adding rules. "1" + 1 = "11", "abc" + "def" = "abcdef", bool + bool = int, bool as int
impl Add for RVecData {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.len() == rhs.len() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 + y).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as f32 + y).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 + y).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| x + *y as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x + *y as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 + *y as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 + y).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x + *y as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x + *y as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 + *y as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 + y).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x + *y as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x + *y as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x + *y as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x + *y as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x + *y as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(a.iter().zip(b.iter()).map(|(x, y)| format!("{}{}", x, y)).collect()),
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 + y).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 + y).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 as f32 + y).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 as f64 + y).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 + *y as i32).collect()),
                
                (a, b) => panic!("Cannot add {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] + x).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 + x).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 + x).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 + x).collect()),
                // (RVec::I32(a), RVec::Str(b))  not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(b.iter().map(|x| a[0] + *x as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(b.iter().map(|x| a[0] + *x as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] + x).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 + *x as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 + x).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(b.iter().map(|x| a[0] + *x as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(b.iter().map(|x| a[0] + *x as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 + *x as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] + x).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 + x).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(b.iter().map(|x| a[0] + *x as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(b.iter().map(|x| a[0] + *x as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] + *x as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 + *x as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] + x).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(b.iter().map(|x| a[0] + *x as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(b.iter().map(|x| format!("{}{}", a[0], x)).collect()),
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 + *x).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 + *x).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 + *x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 + *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 + *x as i32).collect()),

                (a, b) => panic!("Cannot add {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if rhs.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().map(|x| x + b[0]).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| *x as i64 + b[0]).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 + b[0]).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 + b[0]).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().map(|x| x + b[0] as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().map(|x| x + b[0] as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| x + b[0]).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| *x as f64 + b[0] as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 + b[0]).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().map(|x| x + b[0] as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().map(|x| x + b[0] as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 + b[0] as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| x + b[0]).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 + b[0]).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().map(|x| x + b[0] as i32 as f32).collect()),
                
                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().map(|x| x + b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| x + b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| x + b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| x + b[0]).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().map(|x| x + b[0] as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| format!("{}{}", x, b[0])).collect()),
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 + *x).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 + *x).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 + *x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 + *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 + *x as i32).collect()),

                (a, b) => panic!("Cannot add {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("Unsupported lengths: {:?} + {:?}", self.len(), rhs.len())
        }
    }
}

// uses python's multiplication rules, eg. "abc" * 3 = "abcabcabc", bool as int, etc.
impl Mul for RVecData {
    type Output = RVecData;

    fn mul(self, rhs: RVecData) -> RVecData {
        if self.len() == rhs.len() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 * *y).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as f32 * *y).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 * *y).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| x * (*y as i32)).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| *x * *y as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 * *y as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 * *y).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x * (*y as i64)).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x * *y as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 * *y as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 * *y).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x * (*y as i32 as f32)).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x * *y as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x * *y as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x * *y as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x * (*y as i64 as f64)).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported
                
                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| (*x as i32) * y).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| (*x as i64) * y).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| (*x as i32 as f32) * y).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| (*x as i64 as f64) * y).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b.iter()).map(|(x, y)| x & y).collect()),
                
                (a, b) => panic!("Cannot multiply {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| *x as i64 * b[0]).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 * b[0]).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0]).collect()),
                (RVecData::I32(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| b[0].repeat(*x as usize)).collect()),
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().map(|x| x * b[0] as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().map(|x| x * b[0] as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0] as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0]).collect()),
                (RVecData::I64(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| b[0].repeat(*x as usize)).collect()),
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().map(|x| x * b[0] as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().map(|x| x * b[0] as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0] as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0]).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().map(|x| x * b[0] as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| x * b[0]).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as i64 as f64).collect()),

                (RVecData::Str(a), RVecData::I32(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),
                (RVecData::Str(a), RVecData::I64(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0].len())).collect()),
                (RVecData::Str(a), RVecData::Bool(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 * x).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 * x).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 * x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 * x).collect()),
                (RVecData::Bool(a), RVecData::Str(b)) => RVecData::Str(b.iter().map(|x| x.repeat(a[0] as usize)).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 * *x as i32).collect()),

                (a, b) => panic!("Cannot multiply {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if rhs.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| *x as i64 * b[0]).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 * b[0]).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0]).collect()),
                (RVecData::I32(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| b[0].repeat(*x as usize)).collect()),
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().map(|x| x * b[0] as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().map(|x| x * b[0] as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0] as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0]).collect()),
                (RVecData::I64(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| b[0].repeat(*x as usize)).collect()),
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().map(|x| x * b[0] as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().map(|x| x * b[0] as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0] as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0]).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().map(|x| x * b[0] as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| x * b[0]).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as i64 as f64).collect()),

                (RVecData::Str(a), RVecData::I32(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),
                (RVecData::Str(a), RVecData::I64(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0].len())).collect()),
                (RVecData::Str(a), RVecData::Bool(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 * x).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 * x).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 * x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 * x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().map(|x| x & b[0]).collect()),

                (a, b) => panic!("Cannot multiply {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("mismatched lengths {} and {}", self.len(), rhs.len());
        }
    }
}

impl Sub for RVecData {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.len() == rhs.len() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 - y).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as f32 - y).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 - y).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| x - *y as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x - *y as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 - *y as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 - y).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x - *y as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x - *y as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 - *y as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 - y).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x - *y as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x - *y as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x - *y as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x - *y as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x - *y as i64 as f64).collect()),
                
                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 - y).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 - y).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 as f32 - y).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 as f64 - y).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 - *y as i32).collect()),

                (a, b) => panic!("Unsupported types: {:?} - {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 - x).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 - x).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 - x).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - x).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 - *x as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 - *x as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 - x).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 - x).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - x).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 - *x as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 - *x as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 - *x as f32).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 - x).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - x).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 - *x as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - *x as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - *x as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - *x as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - x).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - *x as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 - *x).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 - *x).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 - *x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 - *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(b.iter().map(|x| a[0] ^ *x).collect()),

                (a, b) => panic!("Unsupported types: {:?} - {:?}", a.element_type(), b.element_type()),
            }
            
        } else if rhs.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().map(|x| x - b[0]).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| *x as i64 - b[0]).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 - b[0]).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 - b[0]).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().map(|x| *x - b[0] as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().map(|x| *x - b[0] as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| *x - b[0]).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 - b[0]).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 - b[0]).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().map(|x| *x - b[0] as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().map(|x| *x - b[0] as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 - b[0] as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x - b[0]).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 - b[0]).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().map(|x| *x - b[0] as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().map(|x| *x - b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x - b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| *x - b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x - b[0]).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().map(|x| *x - b[0] as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 - *x).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 - *x).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 - *x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 - *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(b.iter().map(|x| a[0] ^ *x).collect()),

                (a, b) => panic!("Unsupported types: {:?} - {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("mismatched lengths {} and {}", self.len(), rhs.len())
        }
    }
}

impl Div for RVecData {
    type Output = RVecData;

    fn div(self, rhs: RVecData) -> RVecData {
        if self.len() == rhs.len() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as f32 / *y as f32).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y as f64).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as f32 / *y).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as f32 / *y as i32 as f32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y as f64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y as f64).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y as i64 as f64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x / *y).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x / *y).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 as f32 / *y as f32).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 as f64 / *y as f64).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 as f32 / *y).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 as f64 / *y).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 as f32 / *y as i32 as f32).collect()),

                (a, b) => panic!("unsupported types {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 / *x as f32).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as f64).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 / *x).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 / *x as i32 as f32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as f64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as f64).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as i64 as f64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(b.iter().map(|x| a[0] / *x as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] / *x).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(b.iter().map(|x| a[0] / *x as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(b.iter().map(|x| a[0] / *x as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] / *x as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] / *x).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(b.iter().map(|x| a[0] / *x as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 / *x as f32).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 / *x as f64).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 / *x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 / *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 / *x as i32 as f32).collect()),

                (a, b) => panic!("unsupported types {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if rhs.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 / b[0] as f32).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as f64).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 / b[0]).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0]).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().map(|x| *x as f32 / b[0] as i32 as f32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as f64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as f64).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0]).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as i64 as f64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().map(|x| *x / b[0] as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x / b[0]).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0]).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().map(|x| *x / b[0] as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().map(|x| *x / b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x / b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x / b[0]).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().map(|x| *x / b[0] as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 / *x as f32).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 / *x as f64).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 / *x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 / *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::F32(vec![a[0] as i32 as f32 / b[0] as i32 as f32]),

                (a, b) => panic!("unsupported types {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("mismatched lengths {} and {}", self.len(), rhs.len())
        }
    }
}

impl Neg for RVecData {
    type Output = RVecData;

    fn neg(self) -> Self::Output {
        match self {
            RVecData::I32(a) => RVecData::I32(a.iter().map(|x| -*x).collect()),
            RVecData::I64(a) => RVecData::I64(a.iter().map(|x| -*x).collect()),
            RVecData::F32(a) => RVecData::F32(a.iter().map(|x| -*x).collect()),
            RVecData::F64(a) => RVecData::F64(a.iter().map(|x| -*x).collect()),
            // RVec::Str(a) not supported
            RVecData::Bool(a) => RVecData::I32(a.iter().map(|x| -(*x as i32)).collect()),

            a => panic!("unsupported type {:?}", a.element_type()),
        }
    }
}

// need to make equivalent methods for RVecData, all element wise
// capitalize()	Converts the first character to upper case
// casefold()	Converts string into lower case
// center()	Returns a centered string
// count()	Returns the number of times a specified value occurs in a string
// encode()	Returns an encoded version of the string
// endswith()	Returns true if the string ends with the specified value
// expandtabs()	Sets the tab size of the string
// find()	Searches the string for a specified value and returns the position of where it was found
// format()	Formats specified values in a string
// format_map()	Formats specified values in a string
// index()	Searches the string for a specified value and returns the position of where it was found
// isalnum()	Returns True if all characters in the string are alphanumeric
// isalpha()	Returns True if all characters in the string are in the alphabet
// isascii()	Returns True if all characters in the string are ascii characters
// isdecimal()	Returns True if all characters in the string are decimals
// isdigit()	Returns True if all characters in the string are digits
// isidentifier()	Returns True if the string is an identifier
// islower()	Returns True if all characters in the string are lower case
// isnumeric()	Returns True if all characters in the string are numeric
// isprintable()	Returns True if all characters in the string are printable
// isspace()	Returns True if all characters in the string are whitespaces
// istitle()	Returns True if the string follows the rules of a title
// isupper()	Returns True if all characters in the string are upper case
// join()	Converts the elements of an iterable into a string
// ljust()	Returns a left justified version of the string
// lower()	Converts a string into lower case
// lstrip()	Returns a left trim version of the string
// maketrans()	Returns a translation table to be used in translations
// partition()	Returns a tuple where the string is parted into three parts
// replace()	Returns a string where a specified value is replaced with a specified value
// rfind()	Searches the string for a specified value and returns the last position of where it was found
// rindex()	Searches the string for a specified value and returns the last position of where it was found
// rjust()	Returns a right justified version of the string
// rpartition()	Returns a tuple where the string is parted into three parts
// rsplit()	Splits the string at the specified separator, and returns a list
// rstrip()	Returns a right trim version of the string
// split()	Splits the string at the specified separator, and returns a list
// splitlines()	Splits the string at line breaks and returns a list
// startswith()	Returns true if the string starts with the specified value
// strip()	Returns a trimmed version of the string
// swapcase()	Swaps cases, lower case becomes upper case and vice versa
// title()	Converts the first character of each word to upper case
// translate()	Returns a translated string
// upper()	Converts a string into upper case
// zfill()	Fills the string with a specified number of 0 values at the beginning

/// A trait for PythonStr that implements string methods, on strings
pub trait PyStringMethods {
    fn capitalize(&self) -> String;
    fn center(&self, width: usize, fill_char: char) -> String;
    fn count(&self, sub: &str) -> usize;
    fn endswith(&self, suffix: &str) -> bool;
    fn startswith(&self, prefix: &str) -> bool;
    fn find(&self, sub: &str) -> Option<usize>;
    // fn join(&self, sep: &str) -> String;
    fn lower(&self) -> String;
    fn upper(&self) -> String;
    fn replace(&self, old: &str, new: &str, count: Option<usize>) -> String;
    fn split(&self, sep: &str) -> Vec<&str>;
    fn strip(&self) -> String;
    fn lstrip(&self) -> String;
    fn rstrip(&self) -> String;
}

impl PyStringMethods for str {
    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().chain(chars.flat_map(|c| c.to_lowercase())).collect(),
        }
    }

    fn center(&self, width: usize, fill_char: char) -> String {
        let len = self.chars().count();
        if width <= len {
            return self.to_owned();
        }
        let pad_left = (width - len) / 2;
        let pad_right = width - len - pad_left;
        format!("{}{}{}", fill_char.to_string().repeat(pad_left), self, fill_char.to_string().repeat(pad_right))
    }

    fn count(&self, sub: &str) -> usize {
        self.matches(sub).count()
    }

    fn endswith(&self, suffix: &str) -> bool {
        self.ends_with(suffix)
    }

    fn startswith(&self, prefix: &str) -> bool {
        self.starts_with(prefix)
    }

    fn find(&self, sub: &str) -> Option<usize> {
        self.find(sub)
    }

    fn lower(&self) -> String {
        self.to_lowercase()
    }

    fn upper(&self) -> String {
        self.to_uppercase()
    }

    fn replace(&self, old: &str, new: &str, count: Option<usize>) -> String {
        self.replacen(old, new, count.unwrap_or(usize::MAX))
    }

    fn split(&self, sep: &str) -> Vec<&str> {
        self.split(sep).collect()
    }

    fn strip(&self) -> String {
        self.trim().to_owned()
    }

    fn lstrip(&self) -> String {
        self.trim_start().to_owned()
    }

    fn rstrip(&self) -> String {
        self.trim_end().to_owned()
    }
}

pub trait VecStringMethods {
    fn capitalize(&self) -> Self;
    fn center(&self, width: usize, fill_char: char) -> Self;
    fn count(&self, sub: &str) -> Self;
    fn endswith(&self, suffix: &str) -> Self;
    fn startswith(&self, prefix: &str) -> Self;
    fn find(&self, sub: &str) -> Self;
    // fn join(&self, iter: impl Iterator<Item = &str>) -> Self;
    fn lower(&self) -> Self;
    fn upper(&self) -> Self;
    fn replace(&self, old: &str, new: &str) -> Self;
    fn split(&self, sep: &str) -> Self;
    fn strip(&self) -> Self;
    fn lstrip(&self) -> Self;
    fn rstrip(&self) -> Self;
}

impl VecStringMethods for RVecData {
    fn capitalize(&self) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.capitalize()).collect()),
            _ => panic!("capitalize() called on non-string"),
        }
    }

    fn center(&self, width: usize, fill_char: char) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.center(width, fill_char)).collect()),
            _ => panic!("center() called on non-string"),
        }
    }

    fn count(&self, sub: &str) -> Self {
        match self {
            RVecData::Str(s) => RVecData::I32(s.iter().map(|s| s.count(sub) as i32).collect()),
            _ => panic!("count() called on non-string"),
        }
    }

    fn endswith(&self, suffix: &str) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Bool(s.iter().map(|s| s.endswith(suffix)).collect()),
            _ => panic!("endswith() called on non-string"),
        }
    }

    fn startswith(&self, prefix: &str) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Bool(s.iter().map(|s| s.startswith(prefix)).collect()),
            _ => panic!("startswith() called on non-string"),
        }
    }

    fn find(&self, sub: &str) -> Self {
        match self {
            RVecData::Str(s) => RVecData::I32(s.iter().map(|s| s.find(sub).unwrap_or(usize::MAX) as i32).collect()),
            _ => panic!("find() called on non-string"),
        }
    }

    fn lower(&self) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.lower()).collect()),
            _ => panic!("lower() called on non-string"),
        }
    }

    fn upper(&self) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.upper()).collect()),
            _ => panic!("upper() called on non-string"),
        }
    }

    fn replace(&self, old: &str, new: &str) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.replace(old, new)).collect()),
            _ => panic!("replace() called on non-string"),
        }
    }

    fn split(&self, sep: &str) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.split(sep).collect()).collect()),
            _ => panic!("split() called on non-string"),
        }
    }

    fn strip(&self) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.strip()).collect()),
            _ => panic!("strip() called on non-string"),
        }
    }

    fn lstrip(&self) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.lstrip()).collect()),
            _ => panic!("lstrip() called on non-string"),
        }
    }

    fn rstrip(&self) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.rstrip()).collect()),
            _ => panic!("rstrip() called on non-string"),
        }
    }
}