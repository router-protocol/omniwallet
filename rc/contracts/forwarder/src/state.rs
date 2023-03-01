use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use router_wasm_bindings::types::OutboundBatchRequest;

// public constants
pub const CREATE_OUTBOUND_REPLY_ID: u64 = 1;
pub const DEFAULT_EXPIRY_CONFIG: u64 = 5 * 60 * 60; // 5 hours
pub const DEFAULT_GAS_LIMIT: u64 = 300_000;

// static fee for token transfer when fee config is not available
pub const STATIC_FEE: Item<Uint128> = Item::new("static_fee");

// ADMIN address to perform admin priviledged operations
pub const OWNER: Item<Addr> = Item::new("forwarder_contract_owner");

// Custody Smart Contract Addresses Mapping
// (ChainId, ChainType) => CustodySmartContractAddress
pub const CUSTODY_CONTRACT_MAPPING: Map<(String, u32), String> =
    Map::new("custody_contract_mapping");

// Acknowledgement status
pub const ACK_STATUS: Map<&str, String> = Map::new("acknowledgement_status");

// Intermediate State to be used between sub-messages
pub const LAST_OUTBOUND_NONCE: Item<u64> = Item::new("last_outbound_nonce");
pub const TEMP_STATE_CREATE_OUTBOUND_REPLY_ID: Item<Vec<OutboundBatchRequest>> =
    Item::new("temp_state_create_outbound_reply_id");

// OutBound Calls Mapping
pub const OUTBOUND_CALLS_STATE: Map<&str, OutboundBatchRequest> = Map::new("outbound_calls_state");

pub const TEMP_INBOUND_KEY: Item<(String, u32, u64)> = Item::new("temp_inbound_key");

/// It should be in percentage format.
/// Example: if we want to increase gas by 11% then the gas_factor value should be 111
pub const GAS_FACTOR: Item<u64> = Item::new("gas_factor");

pub const GAS_LIMIT: Item<u64> = Item::new("gas_limit");
