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
```

### Installation (Development Mode)
We use `maturin` to effortlessly build the Rust source code into a native Python extension wheel.
```bash

# 1. Create and activate a Python virtual environment
python3 -m venv .venv
source .venv/bin/activate

# 2. Install the Maturin build system
pip install maturin

# 3. Compile and install the Rust extension natively
maturin develop
```

### Usage Example
          
```bash
import distributed_runtime
# Connects to the Rust backend to asynchronously load shards
# without blocking your main training loop.

distributed_runtime.load_dataset("s3://my-bucket/training-data-shard-1")

# Instantly hands off the heavy state dictionary to Rust.
# Rust will multiplex the upload to S3 in the background so
# your expensive GPUs can resume computing the next epoch immediately.

distributed_runtime.save_checkpoint(model_state_dict)
```
