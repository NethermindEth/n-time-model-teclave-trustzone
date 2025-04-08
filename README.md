# One-Time Model PoC in Rust

This project demonstrates a **one-time execution model** using the [Teaclave TrustZone SDK](https://github.com/apache/incubator-teaclave-trustzone-sdk). It provides a Trusted Application (TA) that can only execute a specific operation — **sorting an array of integers** — once, with enforcement handled through OP-TEE secure storage.

This serves as a **proof-of-concept** for building constrained-execution models inside a TEE. The project is built using a minimal `no-std` setup in Rust.

> 🛠️ This project is based on `examples/hello-world-rs` from the upstream [incubator-teaclave-trustzone-sdk](https://github.com/apache/incubator-teaclave-trustzone-sdk), but organized as a standalone project under `projects/one-time-models`. All original `examples` have been removed for clarity and focus.

---

### 📁 Project Structure

There are **three** standalone examples in this project:

- `projects/one-time-sort` — TA that sorts an integer array passed via command line (no token enforcement)
- `projects/reset` — TA and host that reset (delete) a secure counter object by key
- `projects/token_flow` — TA and host implementing full token parsing, signature validation, and one-time execution with sorting

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

4. **Build any project**:

   ```sh
   make -C projects/<one-time-sort | reset | token_flow>
   ```

   > Replace `<...>` with the specific project to compile.

---

### 🚀 Deployment

To deploy built binaries to a target device:

```sh
scp projects/<project>/ta/target/aarch64-unknown-linux-gnu/release/*.ta \
    projects/<project>/host/target/aarch64-unknown-linux-gnu/release/<host_binary> \
    <username>@<host>:<path to install dir>
```

and then on host copy ta file to /lib/optee_armtz:

```sh
sudo cp <path to install dir>/*.ta /lib/optee_armtz 
```
Replace `<project>` and `<host_binary>` with the appropriate directory and binary name.

---

### ▶️ Running the Examples

#### ✅ `one-time-sort`

Sorts a list of integers from the command line. No token validation is performed.

```sh
sudo ./one-time-sort 9 3 7 1 4
```

---

#### 🔒 `token_flow`

Sorts a list of integers once per signed token. The host passes in a `.bin` file that contains a payload + RSA signature.

```sh
# Generate private and public RSA key pair (RSA-2048):
openssl genpkey -algorithm RSA -out private.pem -pkeyopt rsa_keygen_bits:2048
openssl rsa -in private.pem -pubout -out public.pem

# Generate a signed token with sequence number, usage limit, and payload:
cd utils/
cargo run -- token-gen ../private.pem token.bin 9 3 7 1 4

# Run host to submit token and sort integers:
sudo ./token_flow token.bin
```

A second invocation of the host with the same token will be rejected by the TA with `AccessDenied`.

---

#### 🧹 `reset`

Deletes a secure counter object from OP-TEE persistent storage. Useful during development or testing to reset state.

```sh
cd ../projects/reset
make
./host/reset my_counter_key
```

You must pass the same `key` that was used by the `token_flow` TA to create the persistent object.

---

### 🔐 Token Signing Format

Tokens are generated with:

- First 4 bytes: `u32` sequence number (little-endian)
- Next 4 bytes: `u32` usage limit (little-endian)
- Followed by: N × `i32` integers (little-endian) to sort
- RSA-2048 signature (256 bytes) over the message above

The TA extracts the signed payload, verifies it using the embedded public key, and only accepts unique, within-limit tokens.

---

### 💡 Implementation Highlights

- **Secure counter storage** using OP-TEE persistent objects
- **One-time execution enforcement** implemented in the TA via a shared `ExecutionCounter` crate
- **Sorting logic** (bubble sort) runs entirely inside the TA
- **Token verification** with embedded public key and RSA-SHA256 signature check
- **Reset mechanism** to clear persistent state via separate TA
- **Temporary shared memory** (`ParamTmpRef`) to pass data between host and TA

This model can be extended to support **one-time inference**, **limited API calls**, or **time-limited secrets**, serving as a foundation for constrained-use secure applications.

