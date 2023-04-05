use pyo3::prelude::*;

const DEBUG : bool = true;

mod types;
use types::float::FloatVec;
use types::integer::IntVec;
use types::string::StrVec;
use types::bool::BoolVec;

mod utils;
use utils::c;

#[pymodule]
fn rvec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FloatVec>()?;
    m.add_class::<BoolVec>()?;
    m.add_class::<IntVec>()?;
    m.add_class::<StrVec>()?;

    m.add_wrapped(wrap_pyfunction!(c))?;

    Ok(())
}
