def load_dataset(uri: str):
    """
    Register and load a dataset via the Rust runtime.
    """
    print(f"Loading dataset from {uri} (mock)")


import pickle
from ._core import S3Client

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
