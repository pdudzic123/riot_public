mod model;
mod prefiltering;

use prefiltering::Prefiltering;
use pyo3::prelude::*;

#[pymodule]
fn riot(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Prefiltering>()?;
    Ok(())
}
