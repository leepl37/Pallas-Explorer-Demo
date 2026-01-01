# Pallas Explorer Demo

A comprehensive demonstration of the [Pallas](https://github.com/txpipe/pallas) Rust library for Cardano blockchain interaction. This demo showcases the core functionality across Pallas's 4-layer architecture while maintaining compatibility with Pallas v0.34.

## 🏗️ Architecture Overview

Pallas is organized into four conceptual layers:

### Layer 1: The Atoms (The Physics)
- **`pallas-codec`** - CBOR serialization/deserialization (Cardano's binary format)
- **`pallas-crypto`** - Cryptographic primitives (Blake2b, Ed25519, VRF)
- **`pallas-math`** - Precise math utilities (slots, epochs, stake calculations)

### Layer 2: The Data (The Nouns)
- **`pallas-primitives`** - Core data structures (blocks, transactions, headers)
- **`pallas-addresses`** - Address encoding/decoding (Bech32, Base58)

### Layer 3: The Tools (The Verbs)
- **`pallas-traverse`** - Block/transaction analysis and UTxO traversal
- **`pallas-txbuilder`** - Transaction construction and fee calculation

### Layer 4: The Infrastructure (The World)
- **`pallas-network`** - Ouroboros protocol implementation (N2C, N2N)
- **`pallas-hardano`** - Direct ImmutableDB file access
- **`pallas-utxorpc`** - Modern gRPC API for UTxO queries

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ with Cargo
- Optional: Access to a Cardano node for advanced features

### Running the Demo

```bash
# Basic demo (shows codec, crypto, addresses)
cargo run

# With block analysis (requires block CBOR hex)
BLOCK_HEX=your_block_hex_here cargo run

# With network demo (requires Cardano relay access)
CARDANO_RELAY=relay.example.com:3001 cargo run

# With address parsing
ADDR=addr1q9d3h8j3p6f6k3l5m8n2q4r7s9t2v4w6x8y0z cargo run
```

## 📚 Module Demonstrations

### ✅ Included in Demo

| Module | Function | Description |
|--------|----------|-------------|
| `pallas-codec` | `codec_roundtrip()` | CBOR encoding/decoding with minicbor |
| `pallas-crypto` | `crypto_roundtrip()` | Blake2b hashing + Ed25519 signatures |
| `pallas-addresses` | `address_roundtrip()` | Bech32 address parsing and validation |
| `pallas-traverse` | `traverse_roundtrip()` | Multi-era block analysis (Byron→Conway) |
| `pallas-primitives` | *(used in traverse)* | Era-agnostic transaction structures |
| `pallas-network` | `network_roundtrip()` | Ouroboros N2C protocol connection setup |

### ❌ Not Included (API Changes)

These modules are not demonstrated due to API changes in Pallas v0.34:
- **`pallas-txbuilder`** - Transaction construction
- **`pallas-hardano`** - ImmutableDB file reading
- **`pallas-math`** - Slot/epoch math utilities
- **`pallas-utxorpc`** - UTxO-RPC gRPC client

## 🔍 Sample Output

```
== Pallas explorer demo ==
codec: encoded 42 -> 182a
codec: decoded 42
crypto: blake2b256 c6dd40b2321261e845239103414d8e1ff3242728ff75d8d5609b457812b7e860
crypto: signature valid? true
address: failed to parse (error converting from/to bech32 invalid character (code=.)) – set ADDR to a bech32 address
(skip traverse) set BLOCK_HEX to a block's CBOR hex
(skip network) set CARDANO_RELAY=relay:3001 to try ChainSync
```

## 🛠️ Development

### Building
```bash
cargo build
```

### Testing
```bash
cargo test
```

### Dependencies
- **Pallas crates**: v0.34 (aligned across all modules)
- **tokio**: Async runtime for network operations
- **anyhow**: Error handling
- **hex**: Hexadecimal encoding/decoding
- **rand**: Cryptographic random number generation

## 📖 Learning Resources

- [Pallas Documentation](https://docs.rs/pallas/latest/pallas/)
- [Pallas GitHub](https://github.com/txpipe/pallas)
- [Cardano Developer Documentation](https://developers.cardano.org/)
- [Ouroboros Protocol Documentation](https://docs.cardano.org/about-cardano/learn/cardano-node)

## 🤝 Contributing

This is an educational demo project. Feel free to:
- Add examples for the missing modules when APIs stabilize
- Improve error handling and edge cases
- Add more comprehensive tests
- Update to newer Pallas versions

## 📄 License

Licensed under Apache-2.0 (same as Pallas).

---

