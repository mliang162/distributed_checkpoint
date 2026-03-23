def load_dataset(uri: str):
    """
    Register and load a dataset via the Rust runtime.
    """
    print(f"Loading dataset from {uri} (mock)")


import pickle
from ._core import S3Client
from . import _core

def start_coordinator(port: int = 50051):
    """
    Start the gRPC coordinator server on the specified port.
    """
    print(f"Starting python gRPC coordinator on port {port}")
    _core.start_coordinator(port)

def start_worker(worker_id: str, coordinator_addr: str = "http://127.0.0.1:50051"):
    """
    Start a worker node that sends heartbeats to the coordinator.
    """
    print(f"Starting python gRPC worker {worker_id} pointing to {coordinator_addr}")
    _core.start_worker(worker_id, coordinator_addr)
def save_checkpoint(client: S3Client, s3_path: str, state_dict: dict):
    """
    Save an asynchronous checkpoint via the Rust runtime.
    Serializes the heavy state dictionary in Python, then instantly 
    offloads the bytes to Rust for background S3 upload.
    """
    print(f"Serializing checkpoint (size roughly {len(state_dict)} keys)...")
    
    # Serialize the Python object into raw bytes
    raw_bytes = pickle.dumps(state_dict)
    
    print("Handing off byte stream to Rust Tokio engine over FFI...")
    
    # Bypasses the GIL and uploads asynchronously in Rust!
    client.upload_checkpoint(s3_path, raw_bytes)
