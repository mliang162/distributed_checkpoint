use pyo3::prelude::*;

mod checkpoint;
mod coord;
mod io_runtime;
mod storage;

#[pyfunction]
fn hello_world() -> PyResult<String> {
    Ok("Hello from Rust distributed_runtime core!".into())
}

#[pymodule]
fn _core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    m.add_function(wrap_pyfunction!(io_runtime::load_dataset_benchmark, m)?)?;
    m.add_class::<storage::s3_client::S3Client>()?;
    Ok(())
}
