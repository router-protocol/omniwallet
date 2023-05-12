use crate::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub deployer: String,
    pub code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // here user can define other executable messages
    DeployForwarderContract {
        code: String,
        salt: String,
        constructor_args: Vec<String>,
        chain_ids: Vec<String>,
        gas_limits: Vec<u64>,
        gas_prices: Vec<u64>,
    },
    SetForwarderAdmin {
        admin: String,
    },
    SetOwner {
        new_owner: String,
    },
    SetDeployer {
        deployer: String,
    },
    SetCodeId {
        code_id: u64,
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
    FetchForwarderContract { address: String },
    FetchCodeId {},
    FetchDeployer {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeployerExecuteMsg {
    DeployContract {
        code: String,
        salt: String,
        constructor_args: Vec<String>,
        chain_ids: Vec<String>,
        gas_limits: Vec<u64>,
        gas_prices: Vec<u64>,
        forwarder_contract: String,
    },
}
