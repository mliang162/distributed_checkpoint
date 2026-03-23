import multiprocessing
import time
import distributed_runtime

def run_coord():
    print("Starting coordinator...")
    distributed_runtime.start_coordinator(50051)

def run_worker():
    time.sleep(2) # Wait for coord to start
    print("Starting worker...")
    distributed_runtime.start_worker("worker-node-1", "http://127.0.0.1:50051")

if __name__ == '__main__':
    p1 = multiprocessing.Process(target=run_coord)
    p2 = multiprocessing.Process(target=run_worker)

    p1.start()
    p2.start()

    time.sleep(10) # Run for 10 seconds to see heartbeats

    p1.terminate()
    p2.terminate()
    print("Test finished.")
