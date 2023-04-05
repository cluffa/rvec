use pyo3::prelude::*;
use pyo3::types::PyTuple;

use crate::types::float::FloatVec;
use crate::types::integer::IntVec;
use crate::types::string::StrVec;
use crate::types::bool::BoolVec;

/// Concatenates anything to rvec Vectors.
/// the heirarchy is: float -> int -> str -> bool
/// so, starting at float and going down the list,
/// if any of the arguments are of that type,
/// then all of the arguments are converted to that type and concatenated.
/// or it errors if the arguments are of different types.
#[pyfunction(args = "*")]
pub fn c(args: &PyTuple) -> PyResult<PyObject> {
    let types = get_types(args)?;
    if types.contains('f') {
        let mut out = FloatVec::new(vec![]);
        args.into_iter().for_each(|x| {
            if let Ok(x) = x.extract::<FloatVec>() {
                out.data.extend(x.data);
            } else if let Ok(x) = x.extract::<IntVec>() {
                out.data.extend(x.data.iter().map(|x| *x as f64));
            } else if let Ok(x) = x.extract::<Vec<f64>>() {
                out.data.extend(x);
            } else if let Ok(x) = x.extract::<Vec<i64>>() {
                out.data.extend(x.iter().map(|x| *x as f64));
            } else if let Ok(x) = x.extract::<f64>() {
                out.data.push(x);
            } else if let Ok(x) = x.extract::<i64>() {
                out.data.push(x as f64);
            } else {
                panic!("Invalid type for FloatVec: {:?}", x);
            }
        });

        Python::with_gil(|py| Ok(out.into_py(py)))

    } else if types.contains('i') {
        let mut out = IntVec::new(vec![]);
        args.into_iter().for_each(|x| {
            if let Ok(x) = x.extract::<IntVec>() {
                out.data.extend(x.data);
            } else if let Ok(x) = x.extract::<Vec<i64>>() {
                out.data.extend(x);
            } else if let Ok(x) = x.extract::<i64>() {
                out.data.push(x);
            } else {
                panic!("Invalid type for IntVec: {:?}", x);
            }
        });

        Python::with_gil(|py| Ok(out.into_py(py)))
        
    } else if types.contains('b') {
        let mut out = BoolVec::new(vec![]);
        args.into_iter().for_each(|x| {
            if let Ok(x) = x.extract::<BoolVec>() {
                out.data.extend(x.data);
            } else if let Ok(x) = x.extract::<Vec<bool>>() {
                out.data.extend(x);
            } else if let Ok(x) = x.extract::<bool>() {
                out.data.push(x);
            } else {
                panic!("Invalid type for BoolVec: {:?}", x);
            }
        });

        Python::with_gil(|py| Ok(out.into_py(py)))

    } else if types.contains('s') {
        let mut out = StrVec::new(vec![]);
        args.into_iter().for_each(|x| {
            if let Ok(x) = x.extract::<StrVec>() {
                out.data.extend(x.data);
            } else if let Ok(x) = x.extract::<Vec<String>>() {
                out.data.extend(x);
            } else if let Ok(x) = x.extract::<String>() {
                out.data.push(x);
            } else {
                panic!("Invalid type for StrVec: {:?}", x);
            }
        });

        Python::with_gil(|py| Ok(out.into_py(py)))
    } else {
        panic!("Invalid types: {:?}", types);
    }
}

fn get_types(args: &PyTuple) -> PyResult<String> {
    let mut out = String::new();

    args.into_iter().for_each(|x| {
        if let Ok(_) = x.extract::<FloatVec>() {
            out.push('f');
        } else if let Ok(_) = x.extract::<IntVec>() {
            out.push('i');
        } else if let Ok(_) = x.extract::<BoolVec>() {
            out.push('b');
        } else if let Ok(_) = x.extract::<StrVec>() {
            out.push('s');
        } else if let Ok(_) = x.extract::<f64>() {
            out.push('f');
        } else if let Ok(_) = x.extract::<i64>() {
            out.push('i');
        } else if let Ok(_) = x.extract::<bool>() {
            out.push('b');
        } else if let Ok(_) = x.extract::<String>() {
            out.push('s');
        } else if let Ok(_) = x.extract::<Vec<f64>>() {
            out.push('f');
        } else if let Ok(_) = x.extract::<Vec<i64>>() {
            out.push('i');
        } else if let Ok(_) = x.extract::<Vec<bool>>() {
            out.push('b');
        } else if let Ok(_) = x.extract::<Vec<String>>() {
            out.push('s');
        } else {
            panic!("Invalid type: {:?}", x);
        }
    });

    Ok(out)
}