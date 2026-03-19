# Distributed Training Data & Checkpoint Runtime

A high-performance, asynchronous distributed backend for Machine Learning infrastructure. 

This runtime is built to solve the massive network/disk I/O bottlenecks that occur during distributed training (such as downloading 10,000+ data shards or uploading multi-terabyte model checkpoints) by shifting the workload off the Python Global Interpreter Lock (GIL) and into a highly concurrent Rust backend.


In distributed ML training, the most expensive resource, GPUs are frequently left sitting idle. Because standard Python data loaders are crippled by the Global Interpreter Lock (GIL), the cluster's network bandwidth is severely underutilized. As a result, GPUs waste millions of dollars in compute time simply waiting for the network to fetch the next batch of training data or waiting for massive 500GB checkpoints to manually upload to S3.

The Distributed Checkpoint Runtime completely eliminates GPU starvation by shifting network I/O out of Python and into a massive, concurrent Rust engine.


* **Seamless Streaming:** Rust spawns tens of thousands of lightweight tokio tasks that fully saturate the network interface card (NIC), bypassing the Python GIL to stream gigabytes of shards instantly into memory.
* **Background Checkpointing:** Checkpoints are handed off as memory pointers. While Python instantly resumes processing the next epoch, Rust quietly and fault-tolerantly uploads the 500GB state to Amazon S3 in the background.
* **Automatic Coordination:** Lightning-fast gRPC tracking coordinates the worker nodes to ensure zero duplicate downloads and cleanly handles mid-training node failures.

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
