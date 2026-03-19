import os
import time
import concurrent.futures
import distributed_runtime

DATA_DIR = "dataset_mock"

def read_file_python(filepath):
    with open(filepath, "rb") as f:
        return len(f.read())

def benchmark_python(files):
    start = time.time()
    # Read files pure Python with thread pool
    results = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=32) as executor:
        results = list(executor.map(read_file_python, files))
    end = time.time()
    return end - start, len(results)

def benchmark_rust(files):
    start = time.time()
    # Pass off to Rust!
    results = distributed_runtime.load_dataset_benchmark(files)
    end = time.time()
    return end - start, len(results)

if __name__ == "__main__":
    files = [os.path.join(DATA_DIR, f) for f in os.listdir(DATA_DIR) if f.endswith(".txt")]
    print(f"Found {len(files)} files. Starting Benchmark...\n")
    
    # Run Pure Python
    py_time, py_count = benchmark_python(files)
    print(f"Pure Python (ThreadPool): loaded {py_count} files in {py_time:.3f} seconds")
    
    # Run Rust Tokio
    rs_time, rs_count = benchmark_rust(files)
    print(f"Rust (Tokio Async): loaded {rs_count} files in {rs_time:.3f} seconds")
    
    speedup = py_time / rs_time
    print(f"\nRust was {speedup:.2f}x faster off the GIL!")
