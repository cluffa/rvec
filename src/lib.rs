use std::ops::{Add, Sub, Mul, Div, Neg};

use pyo3::{prelude::*, types::PyList};

use nalgebra::DVector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RVT {
    Int,
    Float,
    String,
    Bool,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct RVec {
    pub rt: RVT,
    pub ri: Option<DVector<i64>>,
    pub rf: Option<DVector<f64>>,
    pub rs: Option<DVector<String>>,
    pub rb: Option<DVector<bool>>,
}

#[pymethods]
impl RVec {
    pub fn __len__(&self) -> PyResult<usize> {
        Ok(match self.rt {
            RVT::Int => self.ri.as_ref().unwrap().len(),
            RVT::Float => self.rf.as_ref().unwrap().len(),
            RVT::String => self.rs.as_ref().unwrap().len(),
            RVT::Bool => self.rb.as_ref().unwrap().len(),
        })
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(match self.rt {
            RVT::Int => format!("{:?}", self.ri.as_ref().unwrap()),
            RVT::Float => format!("{:?}", self.rf.as_ref().unwrap()),
            RVT::String => format!("{:?}", self.rs.as_ref().unwrap()),
            RVT::Bool => format!("{:?}", self.rb.as_ref().unwrap()),
        })
    }

    pub fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    pub fn __add__(&self, other: &PyAny) -> PyResult<RVec> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(self.clone() + other)
        } else if let Ok(other) = other.extract::<i64>() {
            Ok(self.clone() + other)
        } else if let Ok(other) = other.extract::<f64>() {
            Ok(self.clone() + other)
        } else if let Ok(other) = other.extract::<String>() {
            Ok(self.clone() + other)
        } else if let Ok(other) = other.extract::<bool>() {
            Ok(self.clone() + other)
        } else {
            panic!("Unsupported type");
        }
    }

    pub fn __sub__(&self, other: &PyAny) -> PyResult<RVec> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(self.clone() - other)
        } else if let Ok(other) = other.extract::<i64>() {
            Ok(self.clone() - other)
        } else if let Ok(other) = other.extract::<f64>() {
            Ok(self.clone() - other)
        } else if let Ok(other) = other.extract::<String>() {
            panic!("Unsupported type");
        } else if let Ok(other) = other.extract::<bool>() {
            panic!("Unsupported type");
        } else {
            panic!("Unsupported type");
        }
    }

    pub fn __mul__(&self, other: &PyAny) -> PyResult<RVec> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(self.clone() * other)
        } else if let Ok(other) = other.extract::<i64>() {
            Ok(self.clone() * other)
        } else if let Ok(other) = other.extract::<f64>() {
            Ok(self.clone() * other)
        } else if let Ok(other) = other.extract::<String>() {
            panic!("Unsupported type");
        } else if let Ok(other) = other.extract::<bool>() {
            panic!("Unsupported type");
        } else {
            panic!("Unsupported type");
        }
    }

    pub fn __truediv__(&self, other: &PyAny) -> PyResult<RVec> {
        if let Ok(other) = other.extract::<RVec>() {
            Ok(self.clone() / other)
        } else if let Ok(other) = other.extract::<i64>() {
            Ok(self.clone() / other)
        } else if let Ok(other) = other.extract::<f64>() {
            Ok(self.clone() / other)
        } else if let Ok(other) = other.extract::<String>() {
            panic!("Unsupported type");
        } else if let Ok(other) = other.extract::<bool>() {
            panic!("Unsupported type");
        } else {
            panic!("Unsupported type");
        }
    }

    pub fn __neg__(&self) -> PyResult<RVec> {
        Ok(-self.clone())
    }
}

#[pyfunction]
pub fn new(x: &PyList) -> PyResult<RVec> {
    // get type of first element
    if x.len() == 0 {
        panic!("Empty list, cannot determine type");
    }

    let py_type = x.get_item(0).unwrap().get_type().name().unwrap();
    let rvt = match py_type {
        "int" => RVT::Int,
        "float" => RVT::Float,
        "str" => RVT::String,
        "bool" => RVT::Bool,
        _ => panic!("Unsupported type"),
    };

    match rvt {
        RVT::Int => {
            let iter = x.iter().map(|x| x.extract::<i64>().unwrap()).collect::<Vec<i64>>();
            Ok(RVec {
                rt: RVT::Int,
                ri: Some(iter.into()),
                rf: None,
                rs: None,
                rb: None,
            })
        },
        RVT::Float => {
            let iter = x.iter().map(|x| x.extract::<f64>().unwrap()).collect::<Vec<f64>>();
            Ok(RVec {
                rt: RVT::Float,
                ri: None,
                rf: Some(iter.into()),
                rs: None,
                rb: None,
            })
        },
        RVT::String => {
            let iter = x.iter().map(|x| x.extract::<String>().unwrap()).collect::<Vec<String>>();
            Ok(RVec {
                rt: RVT::String,
                ri: None,
                rf: None,
                rs: Some(iter.into()),
                rb: None,
            })
        },
        RVT::Bool => {
            let iter = x.iter().map(|x| x.extract::<bool>().unwrap()).collect::<Vec<bool>>();
            Ok(RVec {
                rt: RVT::Bool,
                ri: None,
                rf: None,
                rs: None,
                rb: Some(iter.into()),
            })
        }
    }
}

impl Add<RVec> for RVec {
    type Output = RVec;

    fn add(self, other: RVec) -> RVec {
        match (self.rt, other.rt) {
            (RVT::Int, RVT::Int) => {
                let ri = self.ri.unwrap() + other.ri.unwrap();
                RVec {
                    rt: RVT::Int,
                    ri: Some(ri),
                    rf: None,
                    rs: None,
                    rb: None,
                }
            },
            (RVT::Float, RVT::Float) => {
                let rf = self.rf.unwrap() + other.rf.unwrap();
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            (RVT::Float, RVT::Int) => {
                let rf = self.rf.unwrap() + other.ri.unwrap().cast::<f64>();
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            (RVT::Int, RVT::Float) => {
                let rf = self.ri.unwrap().cast::<f64>() + other.rf.unwrap();
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            // string concatenation
            (RVT::String, RVT::String) => {
                let rs = self.rs.unwrap().iter().zip(other.rs.unwrap().iter()).map(|(x, y)| format!("{}{}", x, y)).collect::<Vec<String>>();
                RVec {
                    rt: RVT::String,
                    ri: None,
                    rf: None,
                    rs: Some(rs.into()),
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}

impl Sub<RVec> for RVec {
    type Output = RVec;

    fn sub(self, other: RVec) -> RVec {
        match (self.rt, other.rt) {
            (RVT::Int, RVT::Int) => {
                let ri = self.ri.unwrap() - other.ri.unwrap();
                RVec {
                    rt: RVT::Int,
                    ri: Some(ri),
                    rf: None,
                    rs: None,
                    rb: None,
                }
            },
            (RVT::Float, RVT::Float) => {
                let rf = self.rf.unwrap() - other.rf.unwrap();
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            (RVT::Float, RVT::Int) => {
                let rf = self.rf.unwrap() - other.ri.unwrap().cast::<f64>();
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            (RVT::Int, RVT::Float) => {
                let rf = self.ri.unwrap().cast::<f64>() - other.rf.unwrap();
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}

impl Mul<RVec> for RVec {
    type Output = RVec;

    fn mul(self, other: RVec) -> RVec {
        match (self.rt, other.rt) {
            (RVT::Int, RVT::Int) => {
                let ri = self.ri.unwrap().component_mul(&other.ri.unwrap());
                RVec {
                    rt: RVT::Int,
                    ri: Some(ri),
                    rf: None,
                    rs: None,
                    rb: None,
                }
            },
            (RVT::Float, RVT::Float) => {
                let rf = self.rf.unwrap().component_mul(&other.rf.unwrap());
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            (RVT::Float, RVT::Int) => {
                let rf = self.rf.unwrap().component_mul(&other.ri.unwrap().cast::<f64>());
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            (RVT::Int, RVT::Float) => {
                let rf = self.ri.unwrap().cast::<f64>().component_mul(&other.rf.unwrap());
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            
            // string repitition
            (RVT::String, RVT::Int) => {
                let rs = self.rs.unwrap().iter().zip(other.ri.unwrap().iter()).map(|(x, y)| x.repeat(*y as usize)).collect::<Vec<String>>();
                RVec {
                    rt: RVT::String,
                    ri: None,
                    rf: None,
                    rs: Some(rs.into()),
                    rb: None,
                }
            },
            (RVT::Int, RVT::String) => {
                let rs = self.ri.unwrap().iter().zip(other.rs.unwrap().iter()).map(|(x, y)| y.repeat(*x as usize)).collect::<Vec<String>>();
                RVec {
                    rt: RVT::String,
                    ri: None,
                    rf: None,
                    rs: Some(rs.into()),
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}
    
impl Div<RVec> for RVec {
    type Output = RVec;

    fn div(self, other: RVec) -> RVec {
        match (self.rt, other.rt) {
            (RVT::Int, RVT::Int) => {
                let ri = self.ri.unwrap().component_div(&other.ri.unwrap());
                RVec {
                    rt: RVT::Int,
                    ri: Some(ri),
                    rf: None,
                    rs: None,
                    rb: None,
                }
            },
            (RVT::Float, RVT::Float) => {
                let rf = self.rf.unwrap().component_div(&other.rf.unwrap());
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            (RVT::Float, RVT::Int) => {
                let rf = self.rf.unwrap().component_div(&other.ri.unwrap().cast::<f64>());
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            (RVT::Int, RVT::Float) => {
                let rf = self.ri.unwrap().cast::<f64>().component_div(&other.rf.unwrap());
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}

impl Add<i64> for RVec {
    type Output = RVec;

    fn add(self, other: i64) -> RVec {
        match self.rt {
            RVT::Int => {
                let ri = self.ri.unwrap().add_scalar(other);
                RVec {
                    rt: RVT::Int,
                    ri: Some(ri),
                    rf: None,
                    rs: None,
                    rb: None,
                }
            },
            RVT::Float => {
                let rf = self.rf.unwrap().add_scalar(other as f64);
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}

impl Sub<i64> for RVec {
    type Output = RVec;

    fn sub(self, other: i64) -> RVec {
        match self.rt {
            RVT::Int => {
                let ri = self.ri.unwrap().add_scalar(- other);
                RVec {
                    rt: RVT::Int,
                    ri: Some(ri),
                    rf: None,
                    rs: None,
                    rb: None,
                }
            },
            RVT::Float => {
                let rf = self.rf.unwrap().add_scalar(- other as f64);
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}

impl Mul<i64> for RVec {
    type Output = RVec;

    fn mul(self, other: i64) -> RVec {
        match self.rt {
            RVT::Int => {
                let ri = self.ri.unwrap() * other;
                RVec {
                    rt: RVT::Int,
                    ri: Some(ri),
                    rf: None,
                    rs: None,
                    rb: None,
                }
            },
            RVT::Float => {
                let rf = self.rf.unwrap() * other as f64;
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}

impl Div<i64> for RVec {
    type Output = RVec;

    fn div(self, other: i64) -> RVec {
        match self.rt {
            RVT::Int => {
                let ri = self.ri.unwrap() / other;
                RVec {
                    rt: RVT::Int,
                    ri: Some(ri),
                    rf: None,
                    rs: None,
                    rb: None,
                }
            },
            RVT::Float => {
                let rf = self.rf.unwrap() / other as f64;
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}

impl Add<f64> for RVec {
    type Output = RVec;

    fn add(self, other: f64) -> RVec {
        match self.rt {
            RVT::Int => {
                let rf = self.ri.unwrap().cast::<f64>().add_scalar(other);
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            RVT::Float => {
                let rf = self.rf.unwrap().add_scalar(other);
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}

impl Sub<f64> for RVec {
    type Output = RVec;

    fn sub(self, other: f64) -> RVec {
        match self.rt {
            RVT::Int => {
                let rf = self.ri.unwrap().cast::<f64>().add_scalar(- other);
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            RVT::Float => {
                let rf = self.rf.unwrap().add_scalar(- other);
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}

impl Mul<f64> for RVec {
    type Output = RVec;

    fn mul(self, other: f64) -> RVec {
        match self.rt {
            RVT::Int => {
                let rf = self.ri.unwrap().cast::<f64>() * other;
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            RVT::Float => {
                let rf = self.rf.unwrap() * other;
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}

impl Div<f64> for RVec {
    type Output = RVec;

    fn div(self, other: f64) -> RVec {
        match self.rt {
            RVT::Int => {
                let rf = self.ri.unwrap().cast::<f64>() / other;
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            RVT::Float => {
                let rf = self.rf.unwrap() / other;
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}

impl Add<String> for RVec {
    type Output = RVec;

    // concat for each element
    fn add(self, other: String) -> RVec {
        match self.rt {
            RVT::String => {
                let rs = self.rs.unwrap().map(|x| x + &other);
                RVec {
                    rt: RVT::String,
                    ri: None,
                    rf: None,
                    rs: Some(rs),
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}

impl Add<bool> for RVec {
    type Output = RVec;

    // treat as int
    fn add(self, other: bool) -> RVec {
        match self.rt {
            RVT::Int => {
                let ri = self.ri.unwrap().add_scalar(other as i64);
                RVec {
                    rt: RVT::Int,
                    ri: Some(ri),
                    rf: None,
                    rs: None,
                    rb: None,
                }
            },
            RVT::Bool => {
                let ri = self.rb.unwrap().map(|x| x as i64).add_scalar(other as i64);
                RVec {
                    rt: RVT::Int,
                    ri: Some(ri),
                    rf: None,
                    rs: None,
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}


impl Neg for RVec {
    type Output = RVec;

    fn neg(self) -> RVec {
        match self.rt {
            RVT::Int => {
                let ri = - self.ri.unwrap();
                RVec {
                    rt: RVT::Int,
                    ri: Some(ri),
                    rf: None,
                    rs: None,
                    rb: None,
                }
            },
            RVT::Float => {
                let rf = - self.rf.unwrap();
                RVec {
                    rt: RVT::Float,
                    ri: None,
                    rf: Some(rf),
                    rs: None,
                    rb: None,
                }
            },
            _ => panic!("Unsupported type"),
        }
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn rvec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RVec>()?;
    m.add_wrapped(wrap_pyfunction!(new))?;
    
    Ok(())
}

