use crate::{Deserialize, Serialize};
use cosmwasm_std::Uint128;
use schemars::JsonSchema;

/// Struct representing transfer information.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransferInfo {
    /// The token address for the transfer.
    pub token_address: String,
    /// The recipient address for the transfer.
    pub recipient: String,
    /// The amount of tokens to be transferred.
    pub amount: Uint128,
    /// Flag indicating whether the token is native to the chain.
    pub is_native: bool,
}

/// Struct representing custody contract information.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CustodyContractInfo {
    /// The address of the custody contract.
    pub address: String,
    /// The chain ID of the custody contract.
    pub chain_id: String,
}

/// Struct representing chain type information.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ChainTypeInfo {
    /// The chain ID.
    pub chain_id: String,
    /// The chain type.
    pub chain_type: u64,
}

/// Struct representing the instantiation message for the smart contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// The address of the deployer.
    pub deployer: String,
    /// The address of the owner.
    pub owner: String,
    /// The chain type information.
    pub chain_type_info: Vec<ChainTypeInfo>,
}

/// Enum representing the different types of execute messages for the smart contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Here the user can define other executable messages
    /// InitiateTransfer message to initiate a transfer to a specific chain.
    InitiateTransfer {
        chain_id: String,
        transfers: Vec<TransferInfo>,
    },
    /// SetCustodyContracts message to set the custody contracts.
    SetCustodyContracts {
        custody_contracts: Vec<CustodyContractInfo>,
    },
    /// SetGasLimit message to set the gas limit.
    SetGasLimit { limit: u64 },
    /// SetGasFactor message to set the gas factor.
    SetGasFactor { gas_factor: u64 },
    /// SetOwner message to set the new owner of the smart contract.
    SetOwner { new_owner: String },
    /// SetDeployer message to set the deployer address.
    SetDeployer { deployer: String },
    /// SetChainTypes message to set the chain type information.
    SetChainTypes { chain_type_info: Vec<ChainTypeInfo> },
    /// DeployContract message to deploy a contract.
    DeployContract {
        code: String,
        salt: String,
        constructor_args: Vec<String>,
        chain_ids: Vec<String>,
        gas_limits: Vec<u64>,
        gas_prices: Vec<u64>,
    },
    /// WithdrawFunds message to withdraw funds to a specific recipient.
    WithdrawFunds { recipient: String, amount: Uint128 },
}

/// Struct representing the migration message for the smart contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

/// Enum representing the different types of query messages for the smart contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // Fetch contract version
    GetContractVersion {},
    /// FetchOwner message to fetch the owner of the smart contract.
    FetchOwner {},
    /// FetchCustodyContract message to fetch the custody contract for a specific chain.
    FetchCustodyContract {
        chain_id: String,
    },
    /// FetchAllCustodyContracts message to fetch all custody contracts.
    FetchAllCustodyContracts {},
    /// FetchAckData message to fetch acknowledgment data for a specific nonce.
    FetchAckData {
        nonce: u64,
    },
    /// FetchContractCalls message to fetch contract calls for a specific nonce.
    FetchContractCalls {
        nonce: u64,
    },
    /// FetchTempItem message to fetch a temporary item.
    FetchTempItem {},
    /// FetchRecentOutboundNonce message to fetch the recent outbound nonce.
    FetchRecentOutboundNonce {},
    /// FetchGasPrice message to fetch the gas price for a specific chain.
    FetchGasPrice {
        chain_id: String,
    },
    /// FetchGasLimit message to fetch the gas limit.
    FetchGasLimit {},
    /// FetchGasFactor message to fetch the gas factor.
    FetchGasFactor {},
    /// FetchDeployer message to fetch the deployer address.
    FetchDeployer {},
    /// FetchChainType message to fetch the chain type for a specific chain.
    FetchChainType {
        chain_id: String,
    },
}
