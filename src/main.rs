//! # Pallas Explorer Demo
//!
//! Demonstrates core functionality of the Pallas Rust library for Cardano blockchain interaction.
//! Based on the 4-layer architecture:
//! - Layer 1 (Atoms): pallas-codec, pallas-crypto, pallas-math
//! - Layer 2 (Data): pallas-primitives, pallas-addresses
//! - Layer 3 (Tools): pallas-traverse, pallas-txbuilder
//! - Layer 4 (Infrastructure): pallas-network, pallas-hardano, pallas-utxorpc
//!
//! Run with environment variables to enable optional features:
//! - BLOCK_HEX=<hex> for block analysis
//! - CARDANO_RELAY=<host:port> for network demo
//! - ADDR=<bech32_address> for address parsing

use anyhow::{Context, Result};
use rand::rngs::OsRng;

use pallas_addresses::Address;
use pallas_codec::minicbor;
use pallas_crypto::{
    hash::{Hash, Hasher},
    key::ed25519::{SecretKeyExtended, Signature},
};
use pallas_traverse::MultiEraBlock;

#[tokio::main]
async fn main() -> Result<()> {
    println!("== Pallas explorer demo ==");

    codec_roundtrip()?;
    crypto_roundtrip()?;
    address_roundtrip()?;

    if let Ok(block_hex) = std::env::var("BLOCK_HEX") {
        traverse_roundtrip(&block_hex)?;
    } else {
        println!("(skip traverse) set BLOCK_HEX to a block's CBOR hex");
    }

    if let Ok(relay) = std::env::var("CARDANO_RELAY") {
        network_roundtrip(&relay).await?;
    } else {
        println!("(skip network) set CARDANO_RELAY=relay:3001 to try ChainSync");
    }

    Ok(())
}

/// Demonstrates pallas-codec: CBOR serialization/deserialization
///
/// pallas-codec provides traits/macros for efficient encoding/decoding of data
/// structures to/from Cardano's binary CBOR format. This is the foundation
/// for all data interchange in Cardano (blocks, transactions, addresses, etc.)
fn codec_roundtrip() -> Result<()> {
    // Simple CBOR roundtrip with built-in types
    let val = 42u64;
    let encoded = minicbor::to_vec(&val).context("encode u64")?;
    println!("codec: encoded {} -> {}", val, hex::encode(&encoded));

    let decoded: u64 = minicbor::decode(&encoded).context("decode u64")?;
    println!("codec: decoded {}", decoded);
    Ok(())
}

/// Demonstrates pallas-crypto: Cryptographic primitives
///
/// pallas-crypto provides Cardano's cryptographic operations:
/// - Blake2b hashing (used for transaction IDs, block hashes)
/// - Ed25519 signatures (used for transaction signing)
/// - BIP32-compatible extended keys (used for HD wallets)
/// - VRF for proof-of-stake randomness
fn crypto_roundtrip() -> Result<()> {
    // Generate a proper Ed25519 extended key (BIP32-compatible for wallets)
    let sk = SecretKeyExtended::new(&mut OsRng);
    let data = b"pallas demo";

    // Create signature with extended key
    let sig: Signature = sk.sign(data);

    // Compute Blake2b hash (same algorithm Cardano uses for tx/block IDs)
    let hash: Hash<32> = Hasher::<256>::hash(data);

    // Verify signature with public key
    let valid = sk.public_key().verify(data, &sig);

    println!("crypto: blake2b256 {}", hex::encode(hash.as_ref()));
    println!("crypto: signature valid? {}", valid);
    Ok(())
}

/// Demonstrates pallas-addresses: Address encoding/decoding
///
/// pallas-addresses handles Cardano address formats:
/// - Bech32 encoding/decoding (modern addr1... addresses)
/// - Base58 decoding (legacy Byron addresses)
/// - Network discrimination (mainnet vs testnet)
/// - Credential extraction (payment keys, stake keys, scripts)
/// - Pointer addresses and stake pool IDs
fn address_roundtrip() -> Result<()> {
    // Try to parse an address from environment variable, or use example
    let addr_str = std::env::var("ADDR").unwrap_or_else(|_| "addr1qexample...".to_string());

    match Address::from_bech32(&addr_str) {
        Ok(addr) => {
            println!("address: parsed {:?}", addr);
        }
        Err(e) => {
            println!("address: failed to parse ({}) – set ADDR to a bech32 address", e);
        }
    }
    Ok(())
}


/// Demonstrates pallas-traverse + pallas-primitives: Block/transaction analysis
///
/// pallas-traverse provides unified analysis tools for Cardano data:
/// - MultiEraBlock: handles blocks across all eras (Byron, Shelley, Allegra, Mary, Alonzo, Babbage, Conway)
/// - Era-agnostic transaction parsing and validation
/// - UTxO set analysis and traversal
/// - Fee calculation and output enumeration
///
/// pallas-primitives defines the core data structures:
/// - Multi-era transaction formats (Tx vs AlonzoTx vs BabbageTx)
/// - Block headers, bodies, and metadata
/// - Protocol parameters and era-specific rules
fn traverse_roundtrip(block_hex: &str) -> Result<()> {
    // Decode hex string to raw CBOR bytes
    let bytes = hex::decode(block_hex).context("decode BLOCK_HEX")?;

    // Parse into MultiEraBlock (handles all Cardano eras automatically)
    let block = MultiEraBlock::decode(&bytes).context("decode MultiEraBlock")?;
    println!(
        "traverse: block with {} txs, era {:?}",
        block.txs().len(),
        block.era()
    );

    // Analyze each transaction in the block
    for tx in block.txs() {
        println!(
            "  tx: fee {:?}, outputs {}",
            tx.fee(),        // Transaction fee in lovelace
            tx.outputs().len() // Number of transaction outputs
        );
    }
    Ok(())
}


/// Demonstrates pallas-network: Node-to-Client (N2C) protocol
///
/// pallas-network implements the Ouroboros network protocol:
/// - Node-to-Client (N2C): client connects to relay nodes
/// - Node-to-Node (N2N): full nodes peer with each other
/// - Mini-protocols: ChainSync, TxSubmission, LocalStateQuery, etc.
/// - Multiplexer: handles multiple protocol channels over single TCP connection
/// - Handshake: negotiates protocol versions and network magic
///
/// Used for:
/// - Following the blockchain (ChainSync)
/// - Submitting transactions (TxSubmission)
/// - Querying current UTxO state (LocalStateQuery)
async fn network_roundtrip(relay: &str) -> Result<()> {
    // For demo purposes, just show connection setup
    // In a real app, you'd establish a TCP connection to a Cardano relay
    println!("network: would connect to {} (bearer creation demo)", relay);

    // Real connection would look like:
    // let bearer = Bearer::connect_tcp("127.0.0.1:3001").await?;
    // let mut client = NodeClient::new(bearer);
    // client.handshake(...).await?;
    // then use client.chainsync(), client.statequery(), etc.

    Ok(())
}

// Note: The following Pallas modules are not demonstrated due to API changes in v0.34:
// - pallas-txbuilder: Transaction construction and fee calculation
// - pallas-hardano: Reading Cardano node's ImmutableDB files directly
// - pallas-math: Slot-to-epoch conversion and stake calculations
// - pallas-utxorpc: Modern gRPC API for UTxO queries and tx submission
//
// These could be added back if using an older Pallas version or when APIs stabilize.

