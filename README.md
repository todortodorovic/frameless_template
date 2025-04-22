# Frameless Substrate Template

A minimal Substrate blockchain template without using FRAME, built with the Polkadot SDK. This template demonstrates how to build a blockchain with custom runtime logic without relying on the standard Substrate FRAME system.

## Overview

This project provides a working example of a frameless Substrate runtime implementation. Instead of using the modular FRAME pallets, it implements all the required runtime API traits directly. This approach offers:

- **Simplicity**: Reduced abstractions for easier understanding of Substrate core concepts
- **Customization**: Full control over runtime implementation details
- **Educational Value**: Demonstrates the underlying architecture of Substrate

## Features

- Complete implementation of required Substrate runtime APIs
- Minimalist storage system with custom state management
- Working AURA/GRANDPA consensus with hardcoded authorities for development
- Custom extrinsic format with basic transaction validation
- Implementation of genesis configuration system
- Support for setting values in storage using custom extrinsics

## Prerequisites

- Rust and Cargo (latest stable version recommended)
- Standard build tools for your platform

## Getting Started

### Clone the Repository

```bash
git clone <repository-url>
cd frameless_template
```

### Build the Project

```bash
cargo build --release
```

### Run the Node

```bash
./target/release/solochain-template-node --dev
```

## Runtime Details

The runtime (`runtime/src/lib.rs`) implements all necessary Substrate runtime APIs:

- `Core`: Version information and block execution logic
- `BlockBuilder`: Block construction and finalization
- `TaggedTransactionQueue`: Transaction validation
- `Metadata`: Runtime metadata for RPC services
- `OffchainWorkerApi`: Offchain worker support (minimal implementation)
- `SessionKeys`: Session key generation and management
- `AuraApi` and `GrandpaApi`: Consensus-related functions
- `AccountNonceApi`: Account nonce tracking
- `GenesisBuilder`: Custom chain genesis state building

### Storage System

The runtime implements a simple but effective storage system:
- `get_state<T>`: Retrieves and decodes values from storage
- `mutate_state<T>`: Updates values in storage with custom logic
- Account nonce tracking with key prefixing

### Custom Extrinsics

Two example extrinsic types are implemented:
- `Call::Foo`: Simple no-op extrinsic
- `Call::SetValue`: Stores a value in runtime storage

## Testing

The project includes comprehensive tests that demonstrate:
- Account nonce handling
- Block lifecycle (initialization, extrinsic application, finalization)
- Storage value persistence
- Transaction validation
- State root calculation

Run tests with:

```bash
cargo test
```


## License

Unlicense (Public Domain)

## Acknowledgements

This template is based on the Polkadot SDK and Substrate framework, developed by Parity Technologies.
