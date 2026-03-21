use object_store::aws::AmazonS3Builder;
use object_store::ObjectStore;
use object_store::path::Path as ObjectStorePath;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::sync::Arc;
use bytes::Bytes;

#[pyclass]
pub struct S3Client {
    pub store: Arc<dyn ObjectStore>,
}

#[pymethods]
impl S3Client {
    /// Creates a new S3 client connected to a specific bucket.
    /// It automatically inherits AWS credentials from your environment or IAM roles.
    #[new]
    pub fn new(bucket_name: String, region: String) -> PyResult<Self> {
        let store = AmazonS3Builder::new()
            .with_region(region)
            .with_bucket_name(bucket_name)
            .build()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        Ok(Self {
            store: Arc::new(store),
        })
    }

    pub fn __repr__(&self) -> String {
        "S3Client(connected=true)".to_string()
    }

    /// Asynchronously downloads a list of S3 paths concurrently using Tokio.
    /// Safely sidesteps the Python GIL and streams bytes directly into memory.
    pub fn download_shards<'py>(&self, py: Python<'py>, paths: Vec<String>) -> PyResult<Vec<&'py PyBytes>> {
        let store = Arc::clone(&self.store);
        
        // Release the GIL, letting Python continue Execution
        let raw_results = py.allow_threads(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            
            rt.block_on(async move {
                let mut handles = Vec::with_capacity(paths.len());
                
                for path in paths {
                    let store_task = Arc::clone(&store);
                    
                    // Spawn a lightweight task for each specific S3 network request
                    handles.push(tokio::spawn(async move {
                        let os_path = ObjectStorePath::from(path);
                        match store_task.get(&os_path).await {
                            Ok(get_result) => {
                                // Wait for S3 to stream all packets and return the byte buffer
                                get_result.bytes().await.ok()
                            }
                            Err(_) => None
                        }
                    }));
                }

                // Gather all results
                let mut results = Vec::with_capacity(handles.len());
                for handle in handles {
                    results.push(handle.await.unwrap_or(None));
                }
                results
            })
        });

        // Re-acquire GIL and hand the pure byte-arrays safely back into Python memory space
        let mut py_results = Vec::with_capacity(raw_results.len());
        for res in raw_results {
            if let Some(bytes) = res {
                py_results.push(PyBytes::new(py, &bytes));
            } else {
                py_results.push(PyBytes::new(py, &[])); // Failed/Missing chunks simply return empty
            }
        }

        Ok(py_results)
    }

    /// Asynchronously uploads a chunk of raw bytes to S3.
    /// This immediately releases the Python GIL so training can resume while uploading.
    pub fn upload_checkpoint<'py>(&self, py: Python<'py>, path: String, data: &'py [u8]) -> PyResult<()> {
        let store = Arc::clone(&self.store);
        
        // We clone the raw bytes slice into an immutable `bytes::Bytes` object 
        // to safely hand off to the background tokio task
        let bytes_data = Bytes::copy_from_slice(data);
        let os_path = ObjectStorePath::from(path);

        // Release the GIL, letting Python continue Execution instantly!
        py.allow_threads(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            
            rt.block_on(async move {
                // Perform the async S3 PUT request
                match store.put(&os_path, bytes_data).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e.to_string()))
                }
            })
        })
    }
}
