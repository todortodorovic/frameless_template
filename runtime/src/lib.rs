//! # Frameless Runtime
//!
//! This crate implements a minimal Substrate runtime **without relying on FRAME macros or pallets**.
//! Instead, it uses core Substrate primitives and pure Rust logic to define a working blockchain runtime.
//!
//! ## Purpose
//!
//! - Explore how Substrate runtimes work "under the hood"
//! - Demonstrate a manually-constructed runtime without FRAME abstraction
//! - Serve as a learning tool or a base for custom runtime experiments
//!
//! ## Features
//!
//! - Manual block initialization, execution, and finalization
//! - Custom extrinsic type (`BasicExtrinsic`) with simple call variants
//! - Minimal support for Aura and Grandpa consensus engines
//! - Compatible with `sp-genesis-builder` for custom genesis configurations
//!
//! ## Use Cases
//!
//! This template is ideal for:
//! - Educational purposes
//! - Lightweight experimentation with runtime internals
//! - Advanced debugging of Substrate behavior without FRAME complexity

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
extern crate alloc;
use log::info;
use parity_scale_codec::{Decode, Encode};
use scale_info::prelude::vec;
use scale_info::TypeInfo;
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_runtime::ExtrinsicInclusionMode;
use sp_runtime::Vec;
use sp_runtime::{
   
    generic::{self},
    impl_opaque_keys,
    traits::{
        BlakeTwo256, Block as BlockT, Extrinsic, Hash, 
    },
    transaction_validity::{
        InvalidTransaction, TransactionSource, TransactionValidity, TransactionValidityError,
        ValidTransaction,
    },
    ApplyExtrinsicResult, BoundToRuntimeAppPublic,
};
use alloc::borrow::Cow;
#[cfg(feature = "std")]
use sp_storage::well_known_keys;
use sp_version::StateVersion;

#[cfg(any(feature = "std", test))]
use sp_runtime::{BuildStorage, Storage};

use sp_core::{ OpaqueMetadata, H256};

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Opaque types and session key definitions used by the node runtime.
///
/// This module defines types that abstract away implementation details of the runtime.
/// They are used by the node for block processing and consensus-related operations.
///
/// ## Components
///
/// - `Header`: Opaque header type using `BlockNumber` and `BlakeTwo256` hashing
/// - `Block`: Opaque block type composed of the header and extrinsics
/// - `SessionKeys`: Session key structure for Aura and Grandpa consensus
/// - `AuraAppPublic` / `GrandpaAppPublic`: Wrappers used to map runtime authorities

pub mod opaque {
    use super::*;
	/// Opaque extrinsic type used by the node.
    type OpaqueExtrinsic = BasicExtrinsic;
  	/// Opaque extrinsic type used by the node.efinition.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type consisting of a header and extrinsics.
    pub type Block = generic::Block<Header, OpaqueExtrinsic>;

    /// Session keys structure used by the runtime for consensus authorities.
    ///
    /// Includes:
    /// - `aura`: Authority key for Aura consensus
    /// - `grandpa`: Authority key for Grandpa finality
    impl_opaque_keys! {
        pub struct SessionKeys {
            pub aura: AuraAppPublic,
            pub grandpa: GrandpaAppPublic,
        }
    }

    /// Wrapper to bind the Aura authority ID to the session key type.
    pub struct AuraAppPublic;
    impl BoundToRuntimeAppPublic for AuraAppPublic {
        type Public = AuraId;
    }

    /// Wrapper to bind the Grandpa authority ID to the session key type.
    pub struct GrandpaAppPublic;
    impl BoundToRuntimeAppPublic for GrandpaAppPublic {
        type Public = sp_consensus_grandpa::AuthorityId;
    }
}

/// Runtime version information used by the Substrate client to check compatibility.
/// Includes specification and implementation versioning, along with supported APIs.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: Cow::Borrowed("frameless-runtime"),
    impl_name: Cow::Borrowed("frameless-runtime"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

/// Returns the native runtime version.
///
/// Used by the native Substrate client to ensure the compiled runtime is compatible
/// with the WASM runtime used on-chain.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

/// Placeholder for the genesis configuration of this runtime.
///
/// This struct can be extended to include any runtime-specific configuration that
/// should be initialized at genesis.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Default))]
pub struct GenesisConfig;

/// Implements the `BuildStorage` trait to provide the initial on-chain storage state.
///
/// Inserts the WASM runtime binary into the `:code` storage key during chain initialization.
#[cfg(feature = "std")]
impl BuildStorage for GenesisConfig {
    fn assimilate_storage(&self, storage: &mut Storage) -> Result<(), String> {
        // Insert compiled runtime WASM code into storage under the :code key.
        storage
            .top
            .insert(well_known_keys::CODE.into(), WASM_BINARY.unwrap().to_vec());

        Ok(())
    }
}
/// Basic blockchain type alias representing block numbers.
pub type BlockNumber = u32;

/// Header type for each block using BlakeTwo256 for hashing.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Full block type combining the header and extrinsics.
pub type Block = generic::Block<Header, BasicExtrinsic>;

/// Currency type used in the runtime. Can be adjusted to use a smaller/larger denomination.
pub type Balance = u128;

/// Nonce type, representing the number of transactions sent by an account.
pub type Nonce = u32;

/// Enum representing all possible runtime calls.
///
/// Each variant corresponds to an operation that can be executed via an extrinsic.
/// - `Foo`: Dummy function for testing dispatch.
/// - `SetValue(u32)`: Stores the provided value into runtime storage.
#[derive(
    Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub enum Call {
    Foo,
    SetValue(u32),
}


/// Basic extrinsic type containing only a runtime call.
///
/// This simplified format omits signature, nonce, and other metadata typically found
/// in signed extrinsics. Suitable for lightweight and test-focused runtimes.
#[derive(
    Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]

pub struct BasicExtrinsic {
    pub function: Call,
}


/// Extrinsic trait implementation for `BasicExtrinsic`.
///
/// Allows Substrate to treat this struct as a valid extrinsic type.
/// Returns an instance of `BasicExtrinsic` using the provided call.

impl Extrinsic for BasicExtrinsic {
    type Call = Call;
    type SignaturePayload = ();

    fn new(data: Self::Call, _: Option<Self::SignaturePayload>) -> Option<Self> {
        Some(Self { function: data })
    }
}

/// Trait implementation for mapping the opaque node block type.
impl sp_runtime::traits::GetNodeBlockType for Runtime {
    type NodeBlock = opaque::Block;
}

/// Trait implementation for mapping the runtime's actual block type.
impl sp_runtime::traits::GetRuntimeBlockType for Runtime {
    type RuntimeBlock = Block;
}

/// Target used for logging purposes (e.g. `log::debug!(target = LOG_TARGET, ...)`)
const LOG_TARGET: &'static str = "frameless";

/// Slot time for the Aura consensus engine in milliseconds (3 seconds).
const BLOCK_TIME: u64 = 3000;

/// Keys used in storage to identify stored items.
/// HEADER_KEY: stores current block header.
/// EXTRINSICS_KEY: stores list of extrinsics.
/// VALUE_KEY: used by `SetValue` to store a `u32` in state.
const HEADER_KEY: &[u8] = b"header"; 
const EXTRINSICS_KEY: &[u8] = b"extrinsics";
const VALUE_KEY: &[u8] = b"value";

/// Account identifier used in the runtime — here based on sr25519 public keys.
pub type AccountId = sp_core::sr25519::Public;


/// Genesis state structure used by `sp-genesis-builder`.
/// Only contains one field: `value`, which will be inserted into storage at genesis.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct RuntimeGenesis {
    pub(crate) value: u32,
}
/// The runtime implementation.
///
/// This struct acts as the core of the blockchain runtime. It contains all the logic for
/// block execution, extrinsic application, storage interaction, and transaction validation.
/// Unlike FRAME-based runtimes, all behavior is defined manually via inherent methods.
pub struct Runtime;

/// Standard alias for dispatch result types — `Ok(())` on success, `Err(())` on failure.
///
/// Used throughout the runtime as a simplified result type for extrinsic execution.
type DispatchResult = Result<(), ()>;

/// Implementation of core runtime logic for the `Runtime` struct.
///
/// Provides helper methods for interacting with storage, applying extrinsics, and executing blocks.
/// This is the manual equivalent of what FRAME pallets usually provide.
impl Runtime {
    /// Reads and decodes a value of type `T` from Substrate storage using the given key.
    ///
    /// Returns `Some(T)` if the value exists and can be decoded, or `None` otherwise.
    /// Useful for accessing runtime storage in a type-safe way.
    fn get_state<T: Decode>(key: &[u8]) -> Option<T> {
        sp_io::storage::get(key).and_then(|d| T::decode(&mut &*d).ok())
    }

    /// Reads a value from storage, applies a mutation to it, and writes it back.
    ///
    /// If the value does not exist under the given key, it is initialized with `T::default()`.
    /// Commonly used to update counters, mappings, or other stored data.
    fn mutate_state<T: Decode + Encode + Default>(key: &[u8], update: impl FnOnce(&mut T)) {
        let mut value = Self::get_state(key).unwrap_or_default();
        update(&mut value);
        sp_io::storage::set(key, &value.encode());
    }

    /// Prefix used to generate unique storage keys for account nonces.
    ///
    /// Nonces are stored under keys formatted as `account_nonce/{account_id}`.
    const ACCOUNT_NONCE_PREFIX: &[u8] = b"account_nonce/";

    /// Computes the full storage key for an account's nonce by combining a prefix and the encoded account ID.
    ///
    /// Used internally to access nonce values stored in Substrate storage.
    fn account_nonce_key(account: &AccountId) -> Vec<u8> {
        let mut key = Self::ACCOUNT_NONCE_PREFIX.to_vec();
        key.extend(account.encode());
        key
    }

    /// Returns the nonce value for a given account.
    ///
    /// If the account has no associated nonce in storage, returns `0`.
    /// This function is used to track the number of transactions an account has sent.
    fn account_nonce_of(account: &AccountId) -> Nonce {
        Self::get_state::<Nonce>(&Self::account_nonce_key(account)).unwrap_or_default()
    }
    /// Executes the logic defined in a given extrinsic (Call variant).
    ///
    /// Currently supports:
    /// - `Call::Foo`: Logs the call but performs no state changes.
    /// - `Call::SetValue(u32)`: Stores the provided value into runtime storage.
    ///
    /// Returns `Ok(())` on success.
    fn dispatch_extrinsic(ext: BasicExtrinsic) -> DispatchResult {
        log::debug!(target: LOG_TARGET, "dispatching {:?}", ext);

        match ext.function {
            Call::Foo => {
                log::info!(target: LOG_TARGET, "Foo called");
            }
            Call::SetValue(v) => {
                log::info!(target: LOG_TARGET, "SetValue({}) called", v);
                sp_io::storage::set(VALUE_KEY, &v.encode());
            }
        }

        Ok(())
    }
    /// Initializes a new block by storing its header and clearing previous extrinsics.
    ///
    /// Called by the executor at the beginning of each block. Sets the context for
    /// extrinsic application and future block finalization.
    ///
    /// Returns `ExtrinsicInclusionMode::AllExtrinsics` to indicate full inclusion support.
    pub(crate) fn do_initialize_block(
        header: &<Block as BlockT>::Header,
    ) -> ExtrinsicInclusionMode {
        sp_io::storage::set(&HEADER_KEY, &header.encode());
        sp_io::storage::clear(&EXTRINSICS_KEY);
        ExtrinsicInclusionMode::AllExtrinsics
    }
    /// Finalizes the block by calculating its state root and extrinsics root.
    ///
    /// These values are inserted into the header and returned. This function must
    /// be called after all extrinsics have been applied and any state changes are complete.
    pub(crate) fn do_finalize_block() -> <Block as BlockT>::Header {
        // fetch the header that was given to us at the beginning of the block.
        let mut header = Self::get_state::<<Block as BlockT>::Header>(HEADER_KEY)
            .expect("We initialized with header, it never got mutated, qed");

        sp_io::storage::clear(&HEADER_KEY);

        let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];
        let state_root = sp_core::H256::decode(&mut &raw_state_root[..]).unwrap();

        let extrinsics = Self::get_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY).unwrap_or_default();
        let extrinsics_root = BlakeTwo256::ordered_trie_root(extrinsics, Default::default());

        header.extrinsics_root = extrinsics_root;
        header.state_root = state_root;
        header
    }


    /// Executes all extrinsics in the given block and asserts final root correctness.
    ///
    /// Used when the node replays or validates blocks. This function ensures that
    /// both state root and extrinsics root match the expected values in the block header.
    pub(crate) fn do_execute_block(block: Block) {
        info!(target: LOG_TARGET, "Entering execute_block. block: {:?}", block);

        for extrinsic in block.clone().extrinsics {
            // block import cannot fail.
            Runtime::dispatch_extrinsic(extrinsic).unwrap();
        }

        // check state root
        let raw_state_root = &sp_io::storage::root(StateVersion::V0)[..];
        let state_root = H256::decode(&mut &raw_state_root[..]).unwrap();
      
        assert_eq!(block.header.state_root, state_root);

        // check extrinsics root.
        let extrinsics = block
            .extrinsics
            .into_iter()
            .map(|x| x.encode())
            .collect::<Vec<_>>();
        let extrinsics_root =
            BlakeTwo256::ordered_trie_root(extrinsics, sp_core::storage::StateVersion::V0);
        assert_eq!(block.header.extrinsics_root, extrinsics_root);
    }
    /// Applies a single extrinsic (transaction) to the current block state.
    ///
    /// - Executes the call (currently supports `Foo` and `SetValue`).
    /// - Logs the result of the dispatch.
    /// - Appends the encoded extrinsic to storage under `EXTRINSICS_KEY` for later trie root calculation.
    ///
    /// Returns `Ok(Ok(()))` on success, or propagates any dispatch error.
    pub(crate) fn do_apply_extrinsic(ext: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
		let dispatch_outcome = Runtime::dispatch_extrinsic(ext.clone());

		log::debug!(target: LOG_TARGET, "dispatched {:?}, outcome = {:?}", ext, dispatch_outcome);
	
		Self::mutate_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY, |current| {
			current.push(ext.encode());
		});
	
		dispatch_outcome
    		.map(Ok)
    		.map_err(|_| TransactionValidityError::Invalid(InvalidTransaction::Custom(0)))
	}
    /// Performs basic validation of an incoming transaction before it's accepted into the transaction pool.
    ///
    /// - Logs the transaction source, data, and associated block hash.
    /// - Accepts all transactions as valid by default.
    /// - Uses the encoded call data as the `provides` tag for uniqueness.
    ///
    /// Returns a `ValidTransaction` result.
    pub(crate) fn do_validate_transaction(
        source: TransactionSource,
        tx: <Block as BlockT>::Extrinsic,
        block_hash: <Block as BlockT>::Hash,
    ) -> TransactionValidity {
        log::debug!(
            target: LOG_TARGET,
            "Entering validate_transaction. source: {:?}, tx: {:?}, block hash: {:?}",
            source,
            tx,
            block_hash
        );


        let data = tx.function;
        Ok(ValidTransaction {
            provides: vec![data.encode()],
            ..Default::default()
        })
    }
    /// Generates inherent extrinsics (like timestamp or consensus data) for the block builder.
    ///
    /// - Currently returns an empty vector since this runtime does not use any inherents.
    /// - Can be extended in the future to support things like `pallet_timestamp`.
    pub(crate) fn do_inherent_extrinsics(_: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
        log::debug!(target: LOG_TARGET, "Entering do_inherent_extrinsics");
        Default::default()
	}
    /// Validates the inherent extrinsics included in a block.
    ///
    /// - Currently performs no validation and returns default (success).
    /// - Useful when inherents like timestamps or slot numbers are in use.
	pub(crate) fn do_check_inherents(
        _: Block,
        _: sp_inherents::InherentData,
    ) -> sp_inherents::CheckInherentsResult {
        log::debug!(target: LOG_TARGET, "Entering do_check_inherents");
        Default::default()
    }

    /// Initializes the storage state from a provided genesis configuration.
    ///
    /// - Stores the value from `RuntimeGenesis` under `VALUE_KEY`.
    /// - Used during chain bootstrapping via `sp-genesis-builder`.
    pub(crate) fn do_build_state(runtime_genesis: RuntimeGenesis) -> sp_genesis_builder::Result {
        sp_io::storage::set(&VALUE_KEY, &runtime_genesis.value.encode());
        Ok(())
    }


    /// Returns a JSON-encoded genesis state preset for a given preset ID.
    ///
    /// - `"local_testnet"` and `"development"` yield `value: 84`
    /// - `None` yields default `value: 42`
    /// - All other presets return `None`
    ///
    /// Used by tools like `sp-genesis-builder` for preset selection.
    pub(crate) fn do_get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
        match id {
			Some(preset_id) if preset_id == "local_testnet" || preset_id == "development" => {
				Some(serde_json::to_vec(&RuntimeGenesis { value: 42*2 }).unwrap())
			}
			None => Some(serde_json::to_vec(&RuntimeGenesis { value: 42 }).unwrap()),
			_ => None,
		}
    }

    /// Returns a list of available preset IDs supported by this runtime.
    ///
    /// Used by tooling to show which named presets can be selected at chain genesis.
    pub(crate) fn do_preset_names() -> Vec<sp_genesis_builder::PresetId> {
		vec![
			"development".into(),
			"local_testnet".into(),
		]
	}
}

/// Implementation of Substrate runtime APIs exposed to the outside world.
///
/// These APIs allow the node and external tools (like Polkadot.js or RPC clients) to
/// interact with the runtime to:
/// - Execute and initialize blocks
/// - Apply and validate extrinsics
/// - Generate session keys
/// - Retrieve metadata and versioning
/// - Access Aura and Grandpa consensus authorities
/// - Manage account nonces and genesis state
///
/// Each trait implementation inside this macro corresponds to a specific set of external runtime capabilities.
/// These are required by the Substrate node to interface correctly with the runtime logic.
impl_runtime_apis! {

    /// Core runtime API implementation.
    ///
    /// Provides version information, block initialization, and full block execution.
    /// Required for all Substrate runtimes to allow external block interaction.
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            info!(
                target: LOG_TARGET,
                "Entering execute_block block: {:?} (exts: {})",
                block,
                block.extrinsics.len()
            );
           
            Self::do_execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
            info!(
                target: LOG_TARGET,
                "Entering initialize_block. header: {:?} / version: {:?}", header, VERSION.spec_version
            );
        
            Self::do_initialize_block(header)
        }
    }

    /// Block builder API implementation.
    ///
    /// Handles extrinsic application, block finalization, and inherent logic.
    /// Used by the block production pipeline to build valid blocks.
    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Self::do_apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Self::do_finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            Self::do_inherent_extrinsics(data)
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData
        ) -> sp_inherents::CheckInherentsResult {
            Self::do_check_inherents(block, data)
        }
    }

    /// Transaction validation API.
    ///
    /// Validates incoming transactions before they're added to the transaction pool.
    /// Returns metadata like dependencies and priority.
    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Self::do_validate_transaction(source, tx, block_hash)
        }
    }

    /// Metadata API.
    ///
    /// Returns runtime metadata used by clients and tooling.
    /// In this implementation, returns empty/default metadata.
    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Default::default())
        }
        fn metadata_at_version(_version: u32) -> Option<OpaqueMetadata> {
            Default::default()
        }

        fn metadata_versions() -> sp_std::vec::Vec<u32> {
            Default::default()
        }
    }

    /// Off-chain worker API.
    ///
    /// Entry point for off-chain logic. Currently not used in this runtime.
    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(_header: &<Block as BlockT>::Header) {
            // we do not do anything.
        }
    }

    /// Session keys API.
    ///
    /// Used to generate and decode session keys for Aura and Grandpa authorities.
    /// Required for validator nodes.
    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            info!(target: "frameless", "🖼️ Entering generate_session_keys. seed: {:?}", seed);
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    /// Aura consensus API.
    ///
    /// Provides the list of authorities and slot duration used by Aura.
    /// Required for block authorship and slot production.
    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(BLOCK_TIME)
        }

        fn authorities() -> Vec<AuraId> {
            // Hardcoded authority key 
            let raw_key: [u8; 32] = hex_literal::hex!(
                "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
            );

            vec![AuraId::from(sp_core::sr25519::Public::from_raw(raw_key))]
        }
    }

    /// Grandpa finality API.
    ///
    /// Provides the list of Grandpa authorities and support for key ownership proofs.
    /// Used by nodes participating in block finality.
    impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
            let raw_key: [u8; 32] = hex_literal::hex!(
                "88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee"
            );

            vec![(sp_consensus_grandpa::AuthorityId::from(sp_core::ed25519::Public::from_raw(raw_key)),1)]
        }

        fn current_set_id() -> sp_consensus_grandpa::SetId {
            0u64
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
                <Block as BlockT>::Hash,
                sp_runtime::traits::NumberFor<Block>,
            >,
            _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: sp_consensus_grandpa::SetId,
            _authority_id: sp_consensus_grandpa::AuthorityId,
        ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
            None
        }
    }

    /// Account nonce API.
    ///
    /// Returns the nonce for a given account. Used by clients before submitting transactions.
    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            Runtime::account_nonce_of(&account)
        }
    }

    /// Genesis builder API.
    ///
    /// Allows the runtime to support genesis configuration via presets and raw JSON blobs.
    /// Used by tools like `sp-genesis-builder` and node CLI commands.
    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            let runtime_genesis: RuntimeGenesis = serde_json::from_slice(&config)
			.map_err(|_e| Cow::Borrowed("Invalid JSON blob"))?;
            info!(target: LOG_TARGET, "Entering build_state: {:?}", runtime_genesis);
            Self::do_build_state(runtime_genesis)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            info!(target: LOG_TARGET, "Entering get_preset: {:?}", id);
            Self::do_get_preset(id)
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            info!(target: LOG_TARGET, "Entering preset_names");
            Self::do_preset_names()
        }
    }
}

/// Unit tests for the Frameless Runtime.
///
/// These tests validate the core functionality of the runtime, including:
/// - State access and mutation
/// - Extrinsic execution
/// - Block lifecycle (initialization, execution, finalization)
/// - Genesis configuration
/// - Transaction validation
/// - Inherent handling
#[cfg(test)]
mod tests {
    use super::*;
    use sp_io::TestExternalities;
    use sp_runtime::traits::Header; 
    /// Ensures a newly created account has a nonce value of 0.
    #[test]
    fn account_nonce_should_start_at_zero() {
        let mut ext = TestExternalities::default();
        ext.execute_with(|| {
            let account = AccountId::from([1u8; 32]);
            let nonce = Runtime::account_nonce_of(&account);
            assert_eq!(nonce, 0);
        });
    }

    /// Verifies that the `mutate_state` helper updates nonce values correctly in storage.
    #[test]
    fn mutate_state_should_change_nonce() {
        let mut ext = TestExternalities::default();
        ext.execute_with(|| {
            let account = AccountId::from([2u8; 32]);
            let key = Runtime::account_nonce_key(&account);
            Runtime::mutate_state::<Nonce>(&key, |n| *n += 5);
            let nonce = Runtime::account_nonce_of(&account);
            assert_eq!(nonce, 5);
        });
    }
    /// Validates that block headers are stored on initialization and retrieved on finalization.
    #[test]
    fn block_lifecycle_should_store_and_finalize_header() {
        let mut ext = TestExternalities::default();
        ext.execute_with(|| {
            let header = Header::new(
                1,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            );

            Runtime::do_initialize_block(&header);
            let finalized = Runtime::do_finalize_block();

            assert_eq!(finalized.number, 1);
        });
    }
    /// Confirms that an extrinsic (`Call::Foo`) can be applied successfully.
    #[test]
    fn should_apply_valid_extrinsic() {
        let mut ext = TestExternalities::default();
        ext.execute_with(|| {
            let ext = BasicExtrinsic { function: Call::Foo };
            let result = Runtime::do_apply_extrinsic(ext);
            assert!(result.is_ok());
        });
    }
    /// Verifies that `do_build_state` correctly inserts the value into genesis storage.
    #[test]
    fn should_build_state_with_value() {
        let mut ext = TestExternalities::default();
        ext.execute_with(|| {
            let genesis = RuntimeGenesis { value: 1337 };
            Runtime::do_build_state(genesis).unwrap();

            let stored: u32 = Runtime::get_state(VALUE_KEY).unwrap();
            assert_eq!(stored, 1337);
        });
    }

    /// Ensures that transactions are accepted as valid by the validation logic.
    #[test]
    fn should_validate_transaction() {
        let mut ext = TestExternalities::default();
        ext.execute_with(|| {
            let tx = BasicExtrinsic { function: Call::Foo };
            let result = Runtime::do_validate_transaction(
                TransactionSource::Local,
                tx,
                H256::repeat_byte(1),
            );
            assert!(result.is_ok());
        });
    }

    /// Ensures the runtime returns an empty list of inherent extrinsics by default.
    #[test]
    fn should_return_default_inherent_extrinsics() {
        let mut ext = TestExternalities::default();
        ext.execute_with(|| {
            let inherents = Runtime::do_inherent_extrinsics(sp_inherents::InherentData::default());
            assert_eq!(inherents.len(), 0);
        });
    }

    /// Ensures `do_check_inherents` returns success without errors.
    #[test]
    fn should_return_default_check_inherents() {
        let mut ext = TestExternalities::default();
        ext.execute_with(|| {
            let header = Header::new(
                1,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            );
            Runtime::do_initialize_block(&header);
            let block = Block {
                header,
                extrinsics: vec![],
            };
            let result = Runtime::do_check_inherents(block, sp_inherents::InherentData::default());
            assert!(result.ok());
        });
    }

    /// Verifies correct behavior of the `do_get_preset` and `do_preset_names` functions.
    #[test]
    fn should_return_presets_correctly() {
        let development = Some("development".into());
        let local = Some("local_testnet".into());
        let none = None;

        assert!(Runtime::do_get_preset(&development).is_some());
        assert!(Runtime::do_get_preset(&local).is_some());
        assert!(Runtime::do_get_preset(&none).is_some());
        assert!(Runtime::do_get_preset(&Some("nonexistent".into())).is_none());

        let presets = Runtime::do_preset_names();
		assert_eq!(presets, vec!["development", "local_testnet"]);
    }

    /// Verifies that `do_finalize_block` sets the correct state root after storage changes.
    #[test]
    fn finalize_block_should_set_correct_state_root() {
        let mut ext = TestExternalities::default();
        ext.execute_with(|| {
            // Prepare and initialize header
            let header = Header::new(
                1,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            );
            Runtime::do_initialize_block(&header);

            // Simulate some state change
            sp_io::storage::set(b"some_key", b"some_value");

            // Finalize block and fetch new header
            let finalized_header = Runtime::do_finalize_block();

            // Calculate expected state root
            let expected_root = {
                let raw = &sp_io::storage::root(StateVersion::V0)[..];
                H256::decode(&mut &raw[..]).unwrap()
            };

            assert_eq!(finalized_header.state_root, expected_root);
        });
    }

    /// Ensures `Call::SetValue` stores the expected value in state.
    #[test]
    fn set_value_call_should_store_value() {
        let mut ext = sp_io::TestExternalities::default();
        ext.execute_with(|| {
            let ext = BasicExtrinsic { function: Call::SetValue(1234) };
            let result = Runtime::do_apply_extrinsic(ext);
            assert!(result.is_ok());

            let stored: u32 = Runtime::get_state(VALUE_KEY).unwrap();
            assert_eq!(stored, 1234);
        });
    }

    /// Confirms that values set during a block persist after finalization.
    #[test]
    fn value_should_persist_through_block_lifecycle() {
        let mut ext = sp_io::TestExternalities::default();
        ext.execute_with(|| {
            let header = Header::new(
                1,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            );
            Runtime::do_initialize_block(&header);

            let extrinsic = BasicExtrinsic { function: Call::SetValue(2025) };
            assert!(Runtime::do_apply_extrinsic(extrinsic).is_ok());

            let finalized = Runtime::do_finalize_block();
            assert_eq!(finalized.number, 1);

            let stored: u32 = Runtime::get_state(VALUE_KEY).unwrap();
            assert_eq!(stored, 2025);
        });
    }
    /// Ensures `Call::SetValue` is accepted by the transaction validation logic.
    #[test]
    fn validate_transaction_should_accept_set_value() {
        let mut ext = sp_io::TestExternalities::default();
        ext.execute_with(|| {
            let tx = BasicExtrinsic { function: Call::SetValue(7) };
            let result = Runtime::do_validate_transaction(
                TransactionSource::External,
                tx,
                H256::repeat_byte(2),
            );
            assert!(result.is_ok());
        });
    }
    /// Validates that a block with no extrinsics has an empty extrinsics root.
    #[test]
    fn finalize_block_without_extrinsics_should_have_empty_root() {
        let mut ext = sp_io::TestExternalities::default();
        ext.execute_with(|| {
            let header = Header::new(
                42,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            );
            Runtime::do_initialize_block(&header);

            let finalized = Runtime::do_finalize_block();

            let expected_root =
                BlakeTwo256::ordered_trie_root(Vec::<Vec<u8>>::new(), StateVersion::V0);
            assert_eq!(finalized.extrinsics_root, expected_root);
        });
    }
    /// Confirms that account nonce keys are constructed with the correct prefix.
    #[test]
    fn account_nonce_key_should_be_prefixed_correctly() {
        let account = AccountId::from([9u8; 32]);
        let key = Runtime::account_nonce_key(&account);

        let mut expected = Runtime::ACCOUNT_NONCE_PREFIX.to_vec();
        expected.extend(account.encode());

        assert_eq!(key, expected);
    }
}
