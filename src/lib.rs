use pyo3::prelude::*;

mod vec_data;
mod vec_operations;
mod vec_comparisons;
mod vec_logic;
mod vec_index;

mod string_methods;

/// default percision
type Fdef = f32;
type Idef = i32;

use vec_data::{RVecData, from_py, BaseRVecData};
use vec_comparisons::ElementCmp;
use vec_logic::ElementLogic;
use vec_index::Indexing;
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

    pub fn to_list(&self) -> PyResult<Vec<PyObject>> {
        self.data.to_list()
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

    pub fn __pos__(&self) -> PyResult<Self> {
        Ok(RVec { data: self.data.clone() })
    }

    pub fn __abs__(&self) -> PyResult<Self> {
        panic!("Not implemented") // TODO
    }

    // TODO: comparisons are not working
    // they work when calling them directly, but not when using the operators
    
    // pub fn __eq__(&self, other: &PyAny) -> PyResult<Self> {
    //     if let Ok(other) = other.extract::<RVec>() {
    //         Ok(RVec { data: self.data.eq_ew(&other.data) })
    //     } else {
    //         Ok(RVec { data: self.data.eq_ew(&from_py(&other)?) })
    //     }
    // }

    // pub fn __ne__(&self, other: &PyAny) -> PyResult<Self> {
    //     if let Ok(other) = other.extract::<RVec>() {
    //         Ok(RVec { data: self.data.ne_ew(&other.data) })
    //     } else {
    //         Ok(RVec { data: self.data.ne_ew(&from_py(&other)?) })
    //     }
    // }

    // pub fn __lt__(&self, other: &PyAny) -> PyResult<Self> {
    //     if let Ok(other) = other.extract::<RVec>() {
    //         Ok(RVec { data: self.data.lt_ew(&other.data) })
    //     } else {
    //         Ok(RVec { data: self.data.lt_ew(&from_py(&other)?) })
    //     }
    // }

    // pub fn __le__(&self, other: &PyAny) -> PyResult<Self> {
    //     if let Ok(other) = other.extract::<RVec>() {
    //         Ok(RVec { data: self.data.le_ew(&other.data) })
    //     } else {
    //         Ok(RVec { data: self.data.le_ew(&from_py(&other)?) })
    //     }
    // }

    // pub fn __gt__(&self, other: &PyAny) -> PyResult<Self> {
    //     if let Ok(other) = other.extract::<RVec>() {
    //         Ok(RVec { data: self.data.gt_ew(&other.data) })
    //     } else {
    //         Ok(RVec { data: self.data.gt_ew(&from_py(&other)?) })
    //     }
    // }

    // pub fn __ge__(&self, other: &PyAny) -> PyResult<Self> {
    //     if let Ok(other) = other.extract::<RVec>() {
    //         Ok(RVec { data: self.data.ge_ew(&other.data) })
    //     } else {
    //         Ok(RVec { data: self.data.ge_ew(&from_py(&other)?) })
    //     }
    // }

    pub fn __and__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(RVec { data: self.data.and_ew(&other.data) })
        } else {
            Ok(RVec { data: self.data.and_ew(&from_py(&other)?) })
        }
    }

    pub fn __or__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(RVec { data: self.data.or_ew(&other.data) })
        } else {
            Ok(RVec { data: self.data.or_ew(&from_py(&other)?) })
        }
    }

    pub fn __xor__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(RVec { data: self.data.xor_ew(&other.data) })
        } else {
            Ok(RVec { data: self.data.xor_ew(&from_py(&other)?) })
        }
    }

    pub fn __invert__(&self) -> PyResult<Self> {
        Ok(RVec { data: self.data.not_ew() })
    }

    pub fn __getitem__(&self, index: &PyAny) -> PyResult<Self> {
        if let Ok(index) = index.extract::<RVec>() {
            Ok(RVec { data: self.data.getindex(index.data) })
        } else {
            Ok(RVec { data: self.data.getindex(from_py(&index)?) })
        }
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