use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const INSTANTIATE_REPLY_ID: u64 = 1;

// ADMIN address to perform admin priviledged operations
pub const OWNER: Item<Addr> = Item::new("forwarder_deployer_contract_owner");

pub const DEPLOYER: Item<String> = Item::new("deployer_address");

pub const FORWARDER_CODE_ID: Item<u64> = Item::new("forwarder_code_id");

pub const TEMP_FORWARDER_OWNER: Item<String> = Item::new("temp_forwarder_owner");

// Forwarder Contract Mapping
// UserAddress => ForwarderContractAddress
pub const FORWARDER_CONTRACT_MAPPING: Map<String, String> = Map::new("forwarder_contract_mapping");
