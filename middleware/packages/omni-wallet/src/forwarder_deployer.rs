use crate::{forwarder::ChainTypeInfo, Deserialize, Serialize};
use schemars::JsonSchema;

/// Struct representing the instantiation message for the smart contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// The contract address of the cross-chain solidity deployer.
    pub deployer: String,
    /// The code ID of the forwarder contract.
    pub code_id: u64,
}

/// Enum representing the different types of execute messages for the smart contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// DeployForwarderContract message to deploy a forwarder contract with chain type information.
    DeployForwarderContract { chain_type_info: Vec<ChainTypeInfo> },
    /// SetForwarderAdmin message to set the admin of the forwarder contract.
    SetForwarderAdmin { admin: String },
    /// SetOwner message to set the new owner of the smart contract.
    SetOwner { new_owner: String },
    /// SetDeployer message to set the deployer contract address.
    SetDeployer { deployer: String },
    /// SetCodeId message to set the code ID of the forwarder cosmwasm contract.
    SetCodeId { code_id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

/// Enum representing the different types of query messages for the smart contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// GetContractVersion message to fetch the contract version.
    GetContractVersion {},
    /// FetchOwner message to fetch the owner of the smart contract.
    FetchOwner {},
    /// FetchForwarderContract message to fetch a specific forwarder contract by its address.
    FetchForwarderContract { address: String },
    /// FetchCodeId message to fetch the code ID of the forwarder cosmwasm contract.
    FetchCodeId {},
    /// FetchDeployer message to fetch the deployer address.
    FetchDeployer {},
}

/// Enum representing the different types of execute messages for the deployer smart contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeployerExecuteMsg {
    /// DeployContract message to deploy a contract with the specified code, salt, constructor arguments,
    /// chain IDs, gas limits, and gas prices.
    DeployContract {
        code: String,
        salt: String,
        constructor_args: Vec<String>,
        chain_ids: Vec<String>,
        gas_limits: Vec<u64>,
        gas_prices: Vec<u64>,
    },
}
