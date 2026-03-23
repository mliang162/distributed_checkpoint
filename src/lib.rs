use pyo3::prelude::*;

mod checkpoint;
mod coord;
mod io_runtime;
mod storage;

#[pyfunction]
fn hello_world() -> PyResult<String> {
    Ok("Hello from Rust distributed_runtime core!".into())
}

#[pyfunction]
fn start_coordinator(py: Python, port: u16) -> PyResult<()> {
    py.allow_threads(|| {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        
        rt.block_on(async {
            // Ideally we'd spawn this and not block indefinitely in a real app,
            // but for now we'll block the thread to keep the server alive.
            let _ = coord::server::run_server(port).await;
        });
    });
    Ok(())
}

#[pyfunction]
fn start_worker(py: Python, worker_id: String, coordinator_addr: String) -> PyResult<()> {
    py.allow_threads(|| {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        
        rt.block_on(async {
            let _ = coord::client::run_client(worker_id, coordinator_addr).await;
        });
    });
    Ok(())
}

#[pymodule]
fn _core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    m.add_function(wrap_pyfunction!(start_coordinator, m)?)?;
    m.add_function(wrap_pyfunction!(start_worker, m)?)?;
    m.add_function(wrap_pyfunction!(io_runtime::load_dataset_benchmark, m)?)?;
    m.add_class::<storage::s3_client::S3Client>()?;
    Ok(())
}
