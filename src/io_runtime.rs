use pyo3::prelude::*;
use tokio::fs;

// The asynchronous core that reads files concurrently.
async fn load_files_async(paths: Vec<String>) -> Vec<usize> {
    let mut handles = Vec::with_capacity(paths.len());
    
    // Spawn a lightweight tokio task for EVERY file simultaneously
    for path in paths {
        handles.push(tokio::spawn(async move {
            fs::read(&path).await.map(|b| b.len()).unwrap_or(0)
        }));
    }

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        if let Ok(data_len) = handle.await {
            results.push(data_len);
        }
    }
    results
}

// The PyO3 boundary bridge
#[pyfunction]
pub fn load_dataset_benchmark(py: Python, paths: Vec<String>) -> PyResult<Vec<usize>> {
    // Release the GIL immediately so Python can do other things if it wants to!
    py.allow_threads(|| {
        // Create an ad-hoc Tokio runtime for the benchmark
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        
        Ok(rt.block_on(async {
            load_files_async(paths).await
        }))
    })
}
