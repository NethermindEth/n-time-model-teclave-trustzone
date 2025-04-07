# One-Time Model PoC in Rust

This project demonstrates a **one-time execution model** using the [Teaclave TrustZone SDK](https://github.com/apache/incubator-teaclave-trustzone-sdk). It provides a Trusted Application (TA) that can only execute a specific operation — **sorting an array of integers** — once, with enforcement handled through OP-TEE secure storage.

This serves as a **proof-of-concept** for building constrained-execution models inside a TEE. The project is built using a minimal `no-std` setup in Rust.

> 🛠️ This project is based on `examples/hello-world-rs` from the upstream [incubator-teaclave-trustzone-sdk](https://github.com/apache/incubator-teaclave-trustzone-sdk), but organized as a standalone project under `projects/one-time-models`. All original `examples` have been removed for clarity and focus.

---

### 🔧 Setup and Building

1. **Set up the development environment**:

   ```sh
   ./setup.sh
   ```

   > This script is tailored for Ubuntu and automatically installs required dependencies.

2. **Build OP-TEE libraries**:

   ```sh
   ./build_optee_libraries.sh optee/
   ```

   > This initializes and compiles OP-TEE components in the `optee/` directory.

3. **Configure the environment**:

   ```sh
   source environment
   ```

   > Sets up toolchain and OP-TEE paths needed to build the TA and host.

4. **Build the One-Time Sort Project**:

   ```sh
   make -C projects/one-time-models
   ```

   > This compiles both the host and trusted application using the updated project structure.

---

### ▶️ Running the Example

**TODO**: Full instructions for running the example in QEMU or on real hardware will be provided. The flow will involve:

- Copying the built binaries to the target platform (QEMU shared folder or device filesystem)
- Running the host application with a list of integers as command-line input
- Verifying that the TA correctly sorts the array *only once*
- On a second invocation, the TA returns an `AccessDenied` error, enforcing the one-time execution policy

---

### 💡 Implementation Highlights

- **Secure counter storage** using OP-TEE persistent objects
- **One-time execution enforcement** implemented in the TA via a shared `ExecutionCounter` crate
- **Sorting logic** (bubble sort) runs entirely inside the TA
- **Host-to-TA communication** uses temporary shared memory (`ParamTmpRef`) to pass integer buffers
- **Command-line interface** on the host to input and receive sorted integer sequences

This model can be extended to support **one-time inference**, **limited API calls**, or **time-limited secrets**, serving as a foundation for constrained-use secure applications.

---
