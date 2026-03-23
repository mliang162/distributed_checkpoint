use super::coordinator::coordinator_client::CoordinatorClient;
use super::coordinator::PingRequest;
use std::time::Duration;
use tokio::time::sleep;

pub async fn run_client(worker_id: String, coordinator_addr: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Worker {} attempting to connect to Coordinator at {}", worker_id, coordinator_addr);

    // Endpoint must include the scheme, e.g. "http://127.0.0.1:50051"
    let mut client = CoordinatorClient::connect(coordinator_addr.clone()).await?;

    println!("Worker {} successfully connected to Coordinator.", worker_id);

    loop {
        let request = tonic::Request::new(PingRequest {
            worker_id: worker_id.clone(),
        });

        match client.ping_worker(request).await {
            Ok(_) => {
                println!("Worker {} successfully pinged Coordinator.", worker_id);
            }
            Err(e) => {
                eprintln!("Worker {} failed to ping Coordinator: {}", worker_id, e);
            }
        }

        // Send a heartbeat every 5 seconds
        sleep(Duration::from_secs(5)).await;
    }
}
