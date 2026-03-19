import distributed_runtime

print("Successfully imported distributed_runtime!")

# This function is executing compiled Rust code
result = distributed_runtime.hello_world()
print(f"Rust returned: {result}") 

assert result == "Hello from Rust distributed_runtime core!", f"Unexpected result: {result}"
print("Python/Rust FFI Bridge is working perfectly!")
