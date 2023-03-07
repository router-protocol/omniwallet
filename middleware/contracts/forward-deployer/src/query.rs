use crate::state::{DEPLOYER, FORWARDER_CODE_ID, FORWARDER_CONTRACT_MAPPING};
use cosmwasm_std::{to_binary, Addr, Binary, Deps, Env, StdResult};
use cw2::get_contract_version;
use omni_wallet::forwarder_contract::QueryMsg;
use router_wasm_bindings::RouterQuery;

use crate::state::OWNER;

pub fn handle_query(deps: Deps<RouterQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        QueryMsg::FetchOwner {} => to_binary(&fetch_owner(deps)?),
        QueryMsg::FetchCodeId {} => to_binary(&fetch_code_id(deps)?),
        QueryMsg::FetchForwarderContract { address } => {
            to_binary(&fetch_forwarder_contract(deps, address)?)
        }
        QueryMsg::FetchDeployer {} => to_binary(&fetch_deployer(deps)?),
    }
}

/**
 * @notice Used to fetch the owner address
*/
pub fn fetch_owner(deps: Deps<RouterQuery>) -> StdResult<Addr> {
    OWNER.load(deps.storage)
}

/**
 * @notice Used to fetch the forwarder contract
 * @param   address
*/
pub fn fetch_forwarder_contract(deps: Deps<RouterQuery>, address: String) -> StdResult<String> {
    FORWARDER_CONTRACT_MAPPING.load(deps.storage, address)
}

/**
 * @notice Used to fetch forwarder code id
*/
pub fn fetch_code_id(deps: Deps<RouterQuery>) -> StdResult<u64> {
    FORWARDER_CODE_ID.load(deps.storage)
}

/**
 * @notice Used to fetch gas factor value
*/
pub fn fetch_deployer(deps: Deps<RouterQuery>) -> StdResult<String> {
    Ok(DEPLOYER.load(deps.storage).unwrap().to_string())
}
