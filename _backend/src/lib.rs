use pyo3::prelude::*;

#[pyfunction]
fn custom_add(a: i32, b: i32) -> PyResult<i32> {
    Ok(a + b)
}

#[pymodule(gil_used = false)]
fn _backend(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.clone().add_function(wrap_pyfunction!(custom_add, m)?)?;
    Ok(())
}