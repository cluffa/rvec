use std::ops::Add;

use pyo3::prelude::*;

#[derive(Debug, Clone, Copy)]
enum RVT {
    Int,
    Float,
    String,
    Bool,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct RVec {
    rt: RVT,
    ri: Option<Vec<i64>>,
    rf: Option<Vec<f64>>,
    rs: Option<Vec<String>>,
    rb: Option<Vec<bool>>,
}

#[pyfunction]
pub fn RInt_new() -> RVec {
    RVec {
        rt: RVT::Int,
        ri: Some(Vec::new()),
        rf: None,
        rs: None,
        rb: None,
    }
}

#[pyfunction]
pub fn RFloat_new() -> RVec {
    RVec {
        rt: RVT::Float,
        ri: None,
        rf: Some(Vec::new()),
        rs: None,
        rb: None,
    }
}

#[pyfunction]
pub fn RString_new() -> RVec {
    RVec {
        rt: RVT::String,
        ri: None,
        rf: None,
        rs: Some(Vec::new()),
        rb: None,
    }
}

#[pyfunction]
pub fn RBool_new() -> RVec {
    RVec {
        rt: RVT::Bool,
        ri: None,
        rf: None,
        rs: None,
        rb: Some(Vec::new()),
    }
}

#[pymethods]
impl RVec {
    pub fn len(&self) -> usize {
        match self.rt {
            RVT::Int => self.ri.as_ref().unwrap().len(),
            RVT::Float => self.rf.as_ref().unwrap().len(),
            RVT::String => self.rs.as_ref().unwrap().len(),
            RVT::Bool => self.rb.as_ref().unwrap().len(),
        }
    }
}

impl Add<i64> for RVec {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        let v = match self.rt {
            RVT::Int => self.ri.as_ref().unwrap().iter().map(|x| x + rhs).collect(),
            RVT::Float => self.rf.as_ref().unwrap().iter().map(|x| x + rhs as f64).collect(),
            RVT::String => self.rs.as_ref().unwrap().iter().map(|x| x + &rhs.to_string()).collect(),
            RVT::Bool => self.rb.as_ref().unwrap().iter().map(|x| x || rhs != 0).collect(),
        };

        

    }
}

impl Add<RVec> for RVec {
    type Output = Self;

    fn add(self, rhs: RVec) -> Self::Output {
        // will cast to each element type, if one or the other is a scalar
        // set out as length of non-scalar or self if both are scalars or same length

        // 
        let self_is_scalar = self.len() == 1;
        let rhs_is_scalar = rhs.len() == 1;


        match (self.rt, rhs.rt) {
            // out is a copy of longer vector, s 
            let out = if ()

            (RVT::Int, RVT::Int) => {
                
            }
            _ => panic!("Invalid type"),
        }
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn rvec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(RInt_new))?;
    m.add_wrapped(wrap_pyfunction!(RFloat_new))?;
    m.add_wrapped(wrap_pyfunction!(RString_new))?;
    m.add_wrapped(wrap_pyfunction!(RBool_new))?;
    m.add_class::<RVec>()?;
    
    Ok(())
}