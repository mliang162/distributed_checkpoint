// gRPC master/worker abstractions

pub mod coordinator {
    tonic::include_proto!("coordinator");
}

pub mod server;
pub mod client;
