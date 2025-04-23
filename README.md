# Frameless Substrate Template

A minimal Substrate blockchain template built **without FRAME**, using only the core components of the Polkadot SDK. This project demonstrates how to build a working blockchain runtime by manually implementing all runtime logic and required APIs — ideal for education, experimentation, and deep understanding of Substrate internals.

---

##  Overview

This repository showcases a complete frameless runtime, replacing all FRAME-based abstractions with handcrafted logic. Instead of using predefined pallets and macros, it manually defines:

- Runtime versioning, state management, and extrinsics
- Block initialization/finalization flow
- Consensus key handling (Aura + Grandpa)
- Full runtime API trait implementations

---

##  Why Frameless?

- **Simplicity** – fewer abstractions make the core logic easier to follow
- **Control** – complete ownership over storage, execution flow, and logic
- **Educational** – ideal for developers learning how Substrate works at the lowest level
- **Lightweight** – minimal, portable runtime setup

---

##  Features

-  Full implementation of required Substrate runtime APIs
-  Minimalist storage with `get_state` / `mutate_state` helpers
-  AURA/GRANDPA consensus with hardcoded authorities for development
-  Custom extrinsic format with basic transaction validation
-  Genesis configuration system with support for JSON presets
-  Comprehensive test coverage
-  Supports Rustdoc documentation generation

---

##  Prerequisites

- Rust (latest **stable** toolchain) – install via [rustup](https://rustup.rs)
- Standard build tools (e.g. clang, make, etc.)

---

##  Getting Started

### Clone & Build

```bash
git clone <repository-url>
cd frameless_template
cargo build --release
```

### Run the Node

```bash
./target/release/solochain-template-node --dev
```

### Example: Submit and Read Extrinsic

Submit a `SetValue(1337)` extrinsic:

```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_submitExtrinsic",
  "params":["0x0139050000"]
}' http://localhost:9944
```

Sample output:
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "result":"0x3352fa68933b84048e249fae0502d11acd1379404ec37943b3caf00ed52d09f3"
}
```

Query the stored value (`value` key):

```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"state_getStorage",
  "params":["0x76616c7565"]
}' http://localhost:9944
```

Sample output:
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "result":"0x39050000"
}
```
#### Explanation of Values

- `0x0139050000` – SCALE-encoded extrinsic for `SetValue(1337)`:
  - `01` → Extrinsic version (signed = false)
  - `39 05 00 00` → `1337` in little-endian

- `0x76616c7565` – Hex encoding of the key `"value"` (used in storage)

- `0x39050000` – SCALE-encoded value `1337` (in little-endian)

---

##  Runtime Breakdown

The runtime logic lives in [`runtime/src/lib.rs`](runtime/src/lib.rs) and implements:

- `Core` – runtime versioning, full block execution
- `BlockBuilder` – block construction and finalization
- `TaggedTransactionQueue` – transaction validation
- `Metadata` – runtime metadata exposure
- `OffchainWorkerApi` – offchain entrypoint (placeholder)
- `SessionKeys` – session key generation for validators
- `AuraApi` / `GrandpaApi` – consensus authority configuration
- `AccountNonceApi` – per-account transaction nonce tracking
- `GenesisBuilder` – custom genesis state definition

---

###  Storage System

- `get_state<T>(key)` – Reads and decodes a value from storage
- `mutate_state<T>(key, fn)` – Reads, mutates, and re-stores a value
- Account nonces are stored with prefixed keys

---

###  Custom Extrinsics

```rust
enum Call {
    Foo,               // No-op call
    SetValue(u32),     // Stores value into runtime state
}
```

Extrinsics are submitted using a lightweight `BasicExtrinsic` struct.

---

##  Testing

Run unit tests:

```bash
cargo test
```

Covered scenarios include:
- State reads/writes
- Extrinsic dispatch
- Block initialization & finalization
- Genesis preset loading
- Nonce tracking and validation logic

---

##  Generate Documentation

You can build full HTML docs from the embedded Rustdoc comments using:

```bash
cargo doc --document-private-items --no-deps --open
```

This will open a browser with all public structs, enums, and functions fully documented.

---

##  Who Is This For?

This template is perfect for:
- Developers curious about how Substrate works internally
- Auditors and researchers seeking transparency
- Educators and learners exploring blockchain concepts
- Hackers experimenting with custom runtime logic


---

## 📄 License

Unlicense (Public Domain) – use freely for learning and building.

---

##  Acknowledgements

Based on the Polkadot SDK and Substrate framework by [Parity Technologies](https://www.parity.io/).