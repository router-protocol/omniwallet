use crate::{Deserialize, Serialize};
use cosmwasm_std::Uint128;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransferInfo {
    pub token_address: String,
    pub recipient: String,
    pub amount: Uint128,
    pub is_native: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CustodyContractInfo {
    pub address: String,
    pub chain_id: String,
    pub chain_type: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ChainTypeInfo {
    pub chain_id: String,
    pub chain_type: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub deployer: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // here user can define other executable messages
    InitiateTransfer {
        chain_id: String,
        transfers: Vec<TransferInfo>,
    },
    SetCustodyContracts {
        custody_contracts: Vec<CustodyContractInfo>,
    },
    SetGasLimit {
        limit: u64,
    },
    SetGasFactor {
        gas_factor: u64,
    },
    SetOwner {
        new_owner: String,
    },
    SetDeployer {
        deployer: String,
    },
    SetChainTypes {
        chain_type_info: Vec<ChainTypeInfo>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // fetch contract version
    GetContractVersion {},
    FetchOwner {},
    FetchCustodyContract { chain_id: String },
    FetchAllCustodyContracts {},
    FetchAckData { nonce: u64 },
    FetchContractCalls { nonce: u64 },
    FetchTempItem {},
    FetchRecentOutboundNonce {},
    FetchGasPrice { chain_id: String },
    FetchGasLimit {},
    FetchGasFactor {},
    FetchDeployer {},
    FetchChainType { chain_id: String },
}
