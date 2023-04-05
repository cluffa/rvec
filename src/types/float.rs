use pyo3::{prelude::*, types::PySlice};
use round::round;

#[allow(unused_imports)]
use super::{integer::IntVec, string::StrVec, bool::BoolVec};

#[pyclass]
#[derive(Clone)]
pub struct FloatVec {
    pub data: Vec<f64>,
}

#[pymethods]
impl FloatVec {
    #[new]
    pub fn new(data: Vec<f64>) -> Self {
        Self { data }
    }

    #[pyo3(text_signature = "($self)")]
    fn len(&self) -> PyResult<usize> {
        Ok(self.data.len())
    }

    #[getter]
    pub fn get_data(&self) -> Vec<f64> {
        self.data.clone()
    }

    #[setter]
    pub fn set_data(&mut self, data: Vec<f64>) {
        self.data = data;
    }

    #[staticmethod]
    pub fn zeros(size: usize) -> Self {
        Self {
            data: vec![0.0; size],
        }
    }

    pub fn to_list(&self) -> PyResult<Vec<f64>> {
        Ok(self.data.clone())
    }

    pub fn to_int(&self) -> PyResult<IntVec> {
        Ok(IntVec {
            data: self.data.iter().map(|x| *x as i64).collect(),
        })
    }

    pub fn get(&self, index: usize) -> f64 {
        self.data[index]
    }

    pub fn set(&mut self, index: usize, value: f64) {
        self.data[index] = value;
    }

    pub fn push(&mut self, value: f64) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> f64 {
        self.data.pop().unwrap()
    }

    pub fn append(&mut self, other: &Self) {
        self.data.extend_from_slice(&other.data);
    }

    pub fn extend(&mut self, other: &Self) {
        self.data.extend_from_slice(&other.data);
    }

    pub fn insert(&mut self, index: usize, value: f64) {
        self.data.insert(index, value);
    }

    pub fn remove(&mut self, index: usize) {
        self.data.remove(index);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    pub fn sort(&mut self) {
        self.data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    pub fn sort_reverse(&mut self) {
        self.data.sort_by(|a, b| b.partial_cmp(a).unwrap());
    }

    pub fn sum(&self) -> f64 {
        self.data.iter().sum()
    }

    // error is lenth is not 1
    pub fn to_scalar(&self) -> PyResult<f64> {
        if self.data.len() == 1 {
            Ok(self.data[0])
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Vector must be length 1",
            ))
        }
    }

    // ############### Python ops ############### //
    // using existing Rust methods
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Vector({:?})", self.data))
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("Vector({:?})", self.data))
    }
    
    // can take int, IntVec, Vec<usize>, BoolVec, or empty, returns empty if not one of these
    // outputs a new FloatVec with the selected elements or single element if index is int
    // negative indices are the same as python
    pub fn __getitem__(&self, index: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(index) = index.extract::<i32>() {
                if crate::DEBUG {
                    py.run("print(\"as i32\")", None, None).expect("bang");
                }

                let index = if index >= 0 {
                    index as usize
                } else {
                    (self.data.len() as i32 + index) as usize
                };

                Ok(self.data[index].into_py(py))
            } else if let Ok(index) = index.extract::<IntVec>() {
                py.run("print(\"as IntVec\")", None, None).expect("bang");
                Ok(FloatVec {
                    data: index.data.iter().map(|x| self.data[*x as usize]).collect(),
                }
                .into_py(py))
            } else if let Ok(index) = index.extract::<Vec<i32>>() {
                if crate::DEBUG {
                    py.run("print(\"as Vec<i32>\")", None, None).expect("bang");
                }
                
                // convert to positive indices
                let index = index
                    .iter()
                    .map(|x| {
                        if *x >= 0 {
                            *x as usize
                        } else {
                            (self.data.len() as i32 + x) as usize
                        }
                    })
                    .collect::<Vec<usize>>();

                Ok(FloatVec {
                    data: index.iter().map(|x| self.data[*x as usize]).collect(),
                }
                .into_py(py))
            } else if let Ok(slice) = index.downcast::<PySlice>() {
                if crate::DEBUG {
                    py.run("print(\"as PySlice\")", None, None).expect("bang");
                }

                let mut data = Vec::new();
                let indices = slice.indices(self.data.len() as i64).unwrap();
                let start = indices.start as usize;
                let stop = indices.stop as usize;
                let step = indices.step as usize;

                for i in (start..stop).step_by(step) {
                    data.push(self.data[i]);
                }

                Ok(FloatVec { data }.into_py(py))
            } else if let Ok(index) = index.extract::<BoolVec>() {
                if crate::DEBUG {
                    py.run("print(\"as BoolVec\")", None, None).expect("bang");
                }

                // must be same length
                if index.data.len() != self.data.len() {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "BoolVec must be same length as Vector",
                    ));
                }

                Ok(FloatVec {
                    data: index
                        .data
                        .iter()
                        .enumerate()
                        .filter_map(|(i, x)| if *x { Some(self.data[i]) } else { None })
                        .collect(),
                }
                .into_py(py))
            } else {
                Ok(py.None())
            }
        })
    }

    

    pub fn __setitem__(&mut self, index: usize, value: f64) -> PyResult<()> {
        self.data[index] = value;
        Ok(())
    }

    pub fn __len__(&self) -> PyResult<usize> {
        Ok(self.data.len())
    }

    // math attrs must go to scalar if other is not a Vector
    // supports IntVec using to_float(), and int using as f64
    pub fn __add__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            self.add(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.add(&other.to_float().unwrap())
        } else {
            Ok(self.scalar_add(other.extract::<f64>()?))
        }
    }

    pub fn __radd__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            self.add(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.add(&other.to_float().unwrap())
        } else {
            Ok(self.scalar_add(other.extract::<f64>()?))
        }
    }

    pub fn __sub__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            self.subtract(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.subtract(&other.to_float().unwrap())
        } else {
            Ok(self.scalar_subtract(other.extract::<f64>()?))
        }
    }

    pub fn __rsub__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            self.subtract(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.subtract(&other.to_float().unwrap())
        } else {
            Ok(self.scalar_subtract(other.extract::<f64>()?))
        }
    }

    pub fn __mul__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            self.multiply(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.multiply(&other.to_float().unwrap())
        } else {
            Ok(self.scalar_multiply(other.extract::<f64>()?))
        }
    }

    pub fn __rmul__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            self.multiply(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.multiply(&other.to_float().unwrap())
        } else {
            Ok(self.scalar_multiply(other.extract::<f64>()?))
        }
    }

    pub fn __truediv__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            self.divide(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.divide(&other.to_float().unwrap())
        } else {
            Ok(self.scalar_divide(other.extract::<f64>()?))
        }
    }

    pub fn __rtruediv__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            self.divide(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.divide(&other.to_float().unwrap())
        } else {
            Ok(self.scalar_divide(other.extract::<f64>()?))
        }
    }

    pub fn __neg__(&self) -> Self {
        self.scalar_multiply(-1.0)
    }

    pub fn __pos__(&self) -> Self {
        self.clone()
    }

    pub fn __abs__(&self) -> Self {
        self.abs()
    }

    pub fn __round__(&self, ndigits: Option<usize>) -> Self {
        self.round(ndigits)
    }

    // comparisons can take float or FloatVec, returns BoolVec
    pub fn __eq__(&self, other: &PyAny) -> PyResult<BoolVec> {
        if let Ok(other) = other.extract::<Self>() {
            self.equals(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.equals(&other.to_float().unwrap())
        } else {
            Ok(self.equals_scalar(other.extract::<f64>()?))
        }
    }

    pub fn __ne__(&self, other: &PyAny) -> PyResult<BoolVec> {
        if let Ok(other) = other.extract::<Self>() {
            self.not_equals(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.not_equals(&other.to_float().unwrap())
        } else {
            Ok(self.not_equals_scalar(other.extract::<f64>()?))
        }
    }

    pub fn __lt__(&self, other: &PyAny) -> PyResult<BoolVec> {
        if let Ok(other) = other.extract::<Self>() {
            self.less_than(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.less_than(&other.to_float().unwrap())
        } else {
            Ok(self.less_than_scalar(other.extract::<f64>()?))
        }
    }

    pub fn __le__(&self, other: &PyAny) -> PyResult<BoolVec> {
        if let Ok(other) = other.extract::<Self>() {
            self.less_than_or_equals_to(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.less_than_or_equals_to(&other.to_float().unwrap())
        } else {
            Ok(self.less_than_or_equals_to_scalar(other.extract::<f64>()?))
        }
    }

    pub fn __gt__(&self, other: &PyAny) -> PyResult<BoolVec> {
        if let Ok(other) = other.extract::<Self>() {
            self.greater_than(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.greater_than(&other.to_float().unwrap())
        } else {
            Ok(self.greater_than_scalar(other.extract::<f64>()?))
        }
    }

    pub fn __ge__(&self, other: &PyAny) -> PyResult<BoolVec> {
        if let Ok(other) = other.extract::<Self>() {
            self.greater_than_or_equals_to(&other)
        } else if let Ok(other) = other.extract::<IntVec>() {
            self.greater_than_or_equals_to(&other.to_float().unwrap())
        } else {
            Ok(self.greater_than_or_equals_to_scalar(other.extract::<f64>()?))
        }
    }

    // ############### OPERATIONS ############### //

    #[pyo3(text_signature = "($self, other)")]
    pub fn add(&self, other: &Self) -> PyResult<Self> {
        if self.data.len() != other.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "vectors must have the same length",
            ));
        }
        Ok(Self {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(x, y)| x + y)
                .collect(),
        })
    }

    #[pyo3(text_signature = "($self, scalar)")]
    pub fn scalar_add(&self, scalar: f64) -> Self {
        Self {
            data: self.data.iter().map(|x| x + scalar).collect(),
        }
    }

    #[pyo3(text_signature = "($self, other)")]
    pub fn subtract(&self, other: &Self) -> PyResult<Self> {
        if self.data.len() != other.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "vectors must have the same length",
            ));
        }
        Ok(Self {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(x, y)| x - y)
                .collect(),
        })
    }

    #[pyo3(text_signature = "($self, scalar)")]
    pub fn scalar_subtract(&self, scalar: f64) -> Self {
        Self {
            data: self.data.iter().map(|x| x - scalar).collect(),
        }
    }
    
    #[pyo3(text_signature = "($self, other)")]
    pub fn multiply(&self, other: &Self) -> PyResult<Self> {
        if self.data.len() != other.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "vectors must have the same length",
            ));
        }
        Ok(Self {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(x, y)| x * y)
                .collect(),
        })
    }

    #[pyo3(text_signature = "($self, scalar)")]
    pub fn scalar_multiply(&self, scalar: f64) -> Self {
        Self {
            data: self.data.iter().map(|x| x * scalar).collect(),
        }
    }
    
    #[pyo3(text_signature = "($self, other)")]
    pub fn divide(&self, other: &Self) -> PyResult<Self> {
        if self.data.len() != other.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "vectors must have the same length",
            ));
        }
        Ok(Self {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(x, y)| x / y)
                .collect(),
        })
    }

    #[pyo3(text_signature = "($self, scalar)")]
    pub fn scalar_divide(&self, scalar: f64) -> Self {
        Self {
            data: self.data.iter().map(|x| x / scalar).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn abs(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.abs()).collect(),
        }
    }

    #[pyo3(text_signature = "($self, digits)")]
    pub fn round(&self, digits: Option<usize>) -> Self {
        match digits {
            Some(digits) => Self {
                data: self.data.iter().map(|x| round(x.clone(), digits as i32)).collect(),
            },
            None => Self {
                data: self.data.iter().map(|x| x.round()).collect(),
            },
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn sqrt(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.sqrt()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn square(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x * x).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn log(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.ln()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn exp(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.exp()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn sin(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.sin()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn cos(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.cos()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn tan(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.tan()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn asin(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.asin()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn acos(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.acos()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn atan(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.atan()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn sinh(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.sinh()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn cosh(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.cosh()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn tanh(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.tanh()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn asinh(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.asinh()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn acosh(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.acosh()).collect(),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn atanh(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.atanh()).collect(),
        }
    }

    // ############### COMPARISONS ############### //
    // compare element wise and return a vector of booleans
    #[pyo3(text_signature = "($self, other)")]
    pub fn equals(&self, other: &Self) -> PyResult<BoolVec> {
        if self.data.len() != other.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "vectors must have the same length",
            ));
        }
        Ok(BoolVec {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(x, y)| x == y)
                .collect(),
        })
    }

    #[pyo3(text_signature = "($self, scalar)")]
    pub fn not_equals(&self, other: &Self) -> PyResult<BoolVec> {
        if self.data.len() != other.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "vectors must have the same length",
            ));
        }
        Ok(BoolVec {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(x, y)| x != y)
                .collect(),
        })
    }

    #[pyo3(text_signature = "($self, other)")]
    pub fn greater_than(&self, other: &Self) -> PyResult<BoolVec> {
        if self.data.len() != other.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "vectors must have the same length",
            ));
        }
        Ok(BoolVec {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(x, y)| x > y)
                .collect(),
        })
    }

    #[pyo3(text_signature = "($self, other)")]
    pub fn greater_than_or_equals_to(&self, other: &Self) -> PyResult<BoolVec> {
        if self.data.len() != other.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "vectors must have the same length",
            ));
        }
        Ok(BoolVec {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(x, y)| x >= y)
                .collect(),
        })
    }

    #[pyo3(text_signature = "($self, other)")]
    pub fn less_than(&self, other: &Self) -> PyResult<BoolVec> {
        if self.data.len() != other.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "vectors must have the same length",
            ));
        }
        Ok(BoolVec {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(x, y)| x < y)
                .collect(),
        })
    }

    #[pyo3(text_signature = "($self, other)")]
    pub fn less_than_or_equals_to(&self, other: &Self) -> PyResult<BoolVec> {
        if self.data.len() != other.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "vectors must have the same length",
            ));
        }
        Ok(BoolVec {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(x, y)| x <= y)
                .collect(),
        })
    }

    // scalar comparisons, return a vector of booleans
    #[pyo3(text_signature = "($self, scalar)")]
    pub fn equals_scalar(&self, scalar: f64) -> BoolVec {
        BoolVec {
            data: self.data.iter().map(|x| x == &scalar).collect(),
        }
    }

    #[pyo3(text_signature = "($self, scalar)")]
    pub fn not_equals_scalar(&self, scalar: f64) -> BoolVec {
        BoolVec {
            data: self.data.iter().map(|x| x != &scalar).collect(),
        }
    }

    #[pyo3(text_signature = "($self, scalar)")]
    pub fn greater_than_scalar(&self, scalar: f64) -> BoolVec {
        BoolVec {
            data: self.data.iter().map(|x| x > &scalar).collect(),
        }
    }

    #[pyo3(text_signature = "($self, scalar)")]
    pub fn greater_than_or_equals_to_scalar(&self, scalar: f64) -> BoolVec {
        BoolVec {
            data: self.data.iter().map(|x| x >= &scalar).collect(),
        }
    }

    #[pyo3(text_signature = "($self, scalar)")]
    pub fn less_than_scalar(&self, scalar: f64) -> BoolVec {
        BoolVec {
            data: self.data.iter().map(|x| x < &scalar).collect(),
        }
    }

    #[pyo3(text_signature = "($self, scalar)")]
    pub fn less_than_or_equals_to_scalar(&self, scalar: f64) -> BoolVec {
        BoolVec {
            data: self.data.iter().map(|x| x <= &scalar).collect(),
        }
    }

    // ############### STATS ############### //

    #[pyo3(text_signature = "($self)")]
    pub fn product(&self) -> f64 {
        self.data.iter().product()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn mean(&self) -> f64 {
        self.sum() / self.len().unwrap() as f64
    }

    #[pyo3(text_signature = "($self)")]
    pub fn min(&self) -> f64 {
        *self.data.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn max(&self) -> f64 {
        *self.data.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn argmin(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
            .unwrap()
            .0
    }

    #[pyo3(text_signature = "($self)")]
    pub fn argmax(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
            .unwrap()
            .0
    }

    #[pyo3(text_signature = "($self)")]
    pub fn median(&self) -> f64 {
        let mut data = self.data.clone();
        data.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let mid = data.len() / 2;
        if data.len() % 2 == 0 {
            (data[mid] + data[mid - 1]) / 2.0
        } else {
            data[mid]
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn variance(&self) -> f64 {
        let mean = self.mean();
        self.data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / self.len().unwrap() as f64
    }

    #[pyo3(text_signature = "($self)")]
    pub fn std(&self) -> f64 {
        self.variance().sqrt()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn norm(&self) -> f64 {
        self.data.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }

    #[pyo3(text_signature = "($self, other)")]
    pub fn dot(&self, other: &Self) -> PyResult<f64> {
        if self.data.len() != other.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "vectors must have the same length",
            ));
        }
        Ok(self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(x, y)| x * y)
            .sum())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        self.data.iter().map(|x| x / norm).collect()
    }
}

// ############### Other IMPLS ############### //

impl std::fmt::Display for FloatVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, x) in self.data.iter().enumerate() {
            write!(f, "{}", x)?;
            if i != self.data.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl std::fmt::Debug for FloatVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FloatVec {{ data: {} }}", self)
    }
}

impl std::ops::Add for &FloatVec {
    type Output = FloatVec;

    fn add(self, other: Self) -> Self::Output {
        FloatVec {
            data: self.data.iter().zip(other.data.iter()).map(|(x, y)| x + y).collect(),
        }
    }
}

impl std::ops::AddAssign for FloatVec {
    fn add_assign(&mut self, other: Self) {
        self.data = self.data.iter().zip(other.data.iter()).map(|(x, y)| x + y).collect();
    }
}

impl std::ops::Sub for &FloatVec {
    type Output = FloatVec;

    fn sub(self, other: Self) -> Self::Output {
        FloatVec {
            data: self.data.iter().zip(other.data.iter()).map(|(x, y)| x - y).collect(),
        }
    }
}

impl std::ops::SubAssign for FloatVec {
    fn sub_assign(&mut self, other: Self) {
        self.data = self.data.iter().zip(other.data.iter()).map(|(x, y)| x - y).collect();
    }
}

impl std::ops::Mul for &FloatVec {
    type Output = FloatVec;

    fn mul(self, other: Self) -> Self::Output {
        FloatVec {
            data: self.data.iter().zip(other.data.iter()).map(|(x, y)| x * y).collect(),
        }
    }
}

impl std::ops::MulAssign for FloatVec {
    fn mul_assign(&mut self, other: Self) {
        self.data = self.data.iter().zip(other.data.iter()).map(|(x, y)| x * y).collect();
    }
}

impl std::ops::Div for &FloatVec {
    type Output = FloatVec;

    fn div(self, other: Self) -> Self::Output {
        FloatVec {
            data: self.data.iter().zip(other.data.iter()).map(|(x, y)| x / y).collect(),
        }
    }
}

impl std::ops::DivAssign for FloatVec {
    fn div_assign(&mut self, other: Self) {
        self.data = self.data.iter().zip(other.data.iter()).map(|(x, y)| x / y).collect();
    }
}

impl std::ops::Mul<f64> for &FloatVec {
    type Output = FloatVec;

    fn mul(self, other: f64) -> Self::Output {
        FloatVec {
            data: self.data.iter().map(|x| x * other).collect(),
        }
    }
}

impl std::ops::MulAssign<f64> for FloatVec {
    fn mul_assign(&mut self, other: f64) {
        self.data = self.data.iter().map(|x| x * other).collect();
    }
}

impl std::ops::Div<f64> for &FloatVec {
    type Output = FloatVec;

    fn div(self, other: f64) -> Self::Output {
        FloatVec {
            data: self.data.iter().map(|x| x / other).collect(),
        }
    }
}

impl std::ops::DivAssign<f64> for FloatVec {
    fn div_assign(&mut self, other: f64) {
        self.data = self.data.iter().map(|x| x / other).collect();
    }
}

impl std::ops::Index<usize> for FloatVec {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IntoIterator for FloatVec {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a> IntoIterator for &'a FloatVec {
    type Item = &'a f64;
    type IntoIter = std::slice::Iter<'a, f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl FromIterator<f64> for FloatVec {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        FloatVec {
            data: iter.into_iter().collect(),
        }
    }
}

impl<'a> FromIterator<&'a f64> for FloatVec {
    fn from_iter<T: IntoIterator<Item = &'a f64>>(iter: T) -> Self {
        FloatVec {
            data: iter.into_iter().cloned().collect(),
        }
    }
}

impl From<Vec<f64>> for FloatVec {
    fn from(vec: Vec<f64>) -> Self {
        FloatVec { data: vec }
    }
}

impl From<FloatVec> for Vec<f64> {
    fn from(vec: FloatVec) -> Self {
        vec.data
    }
}

impl<'a> From<&'a FloatVec> for Vec<f64> {
    fn from(vec: &'a FloatVec) -> Self {
        vec.data.clone()
    }
}

impl<'a> From<&'a [f64]> for FloatVec {
    fn from(vec: &'a [f64]) -> Self {
        FloatVec {
            data: vec.to_vec(),
        }
    }
}

impl<'a> From<&'a Vec<f64>> for FloatVec {
    fn from(vec: &'a Vec<f64>) -> Self {
        FloatVec {
            data: vec.clone(),
        }
    }
}

impl<'a> From<&'a mut Vec<f64>> for FloatVec {
    fn from(vec: &'a mut Vec<f64>) -> Self {
        FloatVec {
            data: vec.clone(),
        }
    }
}

impl<'a> From<&'a mut [f64]> for FloatVec {
    fn from(vec: &'a mut [f64]) -> Self {
        FloatVec {
            data: vec.to_vec(),
        }
    }
}

impl<'a> From<&'a mut FloatVec> for FloatVec {
    fn from(vec: &'a mut FloatVec) -> Self {
        FloatVec {
            data: vec.data.clone(),
        }
    }
}

impl<'a> From<&'a FloatVec> for FloatVec {
    fn from(vec: &'a FloatVec) -> Self {
        FloatVec {
            data: vec.data.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = FloatVec::new(vec![4.0, 5.0, 6.0]);
        let c = a.add(&b).unwrap();
        assert_eq!(c.get_data(), vec![5.0, 7.0, 9.0]);

        let d = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let e = FloatVec::new(vec![4.0, 5.0]);
        let result = d.add(&e);
        assert!(result.is_err());
    }

    #[test]
    fn test_subtract() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = FloatVec::new(vec![4.0, 5.0, 6.0]);
        let c = a.subtract(&b).unwrap();
        assert_eq!(c.get_data(), vec![-3.0, -3.0, -3.0]);

        let d = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let e = FloatVec::new(vec![4.0, 5.0]);
        let result = d.subtract(&e);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiply() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = FloatVec::new(vec![4.0, 5.0, 6.0]);
        let c = a.multiply(&b).unwrap();
        assert_eq!(c.get_data(), vec![4.0, 10.0, 18.0]);

        let d = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let e = FloatVec::new(vec![4.0, 5.0]);
        let result = d.multiply(&e);
        assert!(result.is_err());
    }

    #[test]
    fn test_divide() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = FloatVec::new(vec![4.0, 5.0, 6.0]);
        let c = a.divide(&b).unwrap();
        assert_eq!(c.get_data(), vec![0.25, 0.4, 0.5]);

        let d = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let e = FloatVec::new(vec![4.0, 5.0]);
        let result = d.divide(&e);
        assert!(result.is_err());
    }

    #[test]
    fn test_scalar_add() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = a.scalar_add(4.0);
        assert_eq!(b.get_data(), vec![5.0, 6.0, 7.0]);
    }

    #[test]
    fn test_scalar_subtract() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = a.scalar_subtract(4.0);
        assert_eq!(b.get_data(), vec![-3.0, -2.0, -1.0]);
    }

    #[test]
    fn test_scalar_multiply() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = a.scalar_multiply(4.0);
        assert_eq!(b.get_data(), vec![4.0, 8.0, 12.0]);
    }

    #[test]
    fn test_scalar_divide() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = a.scalar_divide(4.0);
        assert_eq!(b.get_data(), vec![0.25, 0.5, 0.75]);
    }

    #[test]
    fn test_dot() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = FloatVec::new(vec![4.0, 5.0, 6.0]);
        let c = a.dot(&b).unwrap();
        assert_eq!(c, 32.0);

        let d = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let e = FloatVec::new(vec![4.0, 5.0]);
        let result = d.dot(&e);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = a.norm();
        assert_eq!(b, 3.7416573867739413);
    }

    #[test]
    fn test_normalize() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = a.normalize();
        assert_eq!(b.get_data(), vec![0.2672612419124244, 0.5345224838248488, 0.8017837257372732]);
    }

    #[test]
    fn test_into_iter() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let mut iter = a.into_iter();
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), Some(2.0));
        assert_eq!(iter.next(), Some(3.0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_from_iter() {
        let a = vec![1.0, 2.0, 3.0];
        let b = FloatVec::from_iter(a);
        assert_eq!(b.get_data(), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_index() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(a[0], 1.0);
        assert_eq!(a[1], 2.0);
        assert_eq!(a[2], 3.0);
    }

    #[test]
    fn test_display() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(format!("{}", a), "[1, 2, 3]");
    }

    #[test]
    fn test_debug() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(format!("{:?}", a), "FloatVec { data: [1, 2, 3] }");
    }

    #[test]
    fn test_add_assign() {
        let mut a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = FloatVec::new(vec![4.0, 5.0, 6.0]);
        a += b;
        assert_eq!(a.get_data(), vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = FloatVec::new(vec![4.0, 5.0, 6.0]);
        a -= b;
        assert_eq!(a.get_data(), vec![-3.0, -3.0, -3.0]);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = FloatVec::new(vec![4.0, 5.0, 6.0]);
        a *= b;
        assert_eq!(a.get_data(), vec![4.0, 10.0, 18.0]);
    }

    #[test]
    fn test_div_assign() {
        let mut a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        let b = FloatVec::new(vec![4.0, 5.0, 6.0]);
        a /= b;
        assert_eq!(a.get_data(), vec![0.25, 0.4, 0.5]);
    }

    #[test]
    fn test_sum() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(a.sum(), 6.0);
    }

    #[test]
    fn test_product() {
        let a = FloatVec::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(a.product(), 6.0);
    }
}
