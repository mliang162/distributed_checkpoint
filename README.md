# Distributed Training Data & Checkpoint Runtime

A high-performance, asynchronous distributed backend for Machine Learning infrastructure. 
This runtime is built to solve the massive network/disk I/O bottlenecks that occur during distributed training (such as downloading 10,000+ data shards or uploading multi-terabyte model checkpoints) by shifting the workload off the Python Global Interpreter Lock (GIL) and into a highly concurrent Rust backend.

## Architecture Paradigm: "Python on Top, Rust Under the Hood"
ML Engineers interact exclusively with a clean, high-level Python API (`distributed_runtime`), which can be dropped directly into PyTorch or JAX training loops. 

Behind the scenes, the Python library uses a Foreign Function Interface (FFI) via **PyO3** to offload the actual I/O tasks to a blazingly fast **Rust** core. The Rust core utilizes the `tokio` asynchronous runtime to saturate the Network Interface Card (NIC) with concurrent requests, seamlessly bypassing Python's thread-blocking limitations.

### Core Tech Stack
* **Language:** Rust (Core engine), Python (User API)
* **FFI Bridge:** PyO3 & Maturin
* **Async Runtime:** Tokio
* **Cloud Storage:** Apache Arrow `object_store`
* **Node Coordination:** gRPC (via `tonic` & `prost`)

## Getting Started

### Prerequisites
You will need both Python 3.8+ and the Rust compiler installed on your system.
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
