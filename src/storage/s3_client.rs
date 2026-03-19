use object_store::aws::AmazonS3Builder;
use object_store::ObjectStore;
use pyo3::prelude::*;
use std::sync::Arc;

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
}
