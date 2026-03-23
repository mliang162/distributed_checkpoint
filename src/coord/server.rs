use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

// Import generated protobuf structures
use super::coordinator::coordinator_server::{Coordinator, CoordinatorServer};
use super::coordinator::{PingRequest, PingResponse};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CoordinatorService {
    // Map of worker_id to last ping time (in seconds since epoch)
    workers: Arc<Mutex<HashMap<String, u64>>>,
}

impl CoordinatorService {
    pub fn new() -> Self {
        Self {
            workers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl Coordinator for CoordinatorService {
    async fn ping_worker(
        &self,
        request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        let req = request.into_inner();
        let worker_id = req.worker_id;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Update the last seen timestamp for this worker
        let mut workers = self.workers.lock().await;
        workers.insert(worker_id.clone(), now);

        println!("Coordinator received ping from worker: {}", worker_id);

        Ok(Response::new(PingResponse { ok: true }))
    }
}

pub async fn run_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let coordinator_service = CoordinatorService::new();

    println!("Starting Coordinator Server on {}", addr);

    Server::builder()
        .add_service(CoordinatorServer::new(coordinator_service))
        .serve(addr)
        .await?;

    Ok(())
}
