use crate::{
    state::{CHAIN_TYPE_MAPPING, CUSTODY_CONTRACT_MAPPING, DEPLOYER, GAS_LIMIT},
    utils::fetch_oracle_gas_price,
};
use cosmwasm_std::{to_binary, Binary, Deps, Env, Order, StdResult};
use cw2::get_contract_version;
use omni_wallet::forwarder::QueryMsg;
use router_wasm_bindings::{RouterMsg, RouterQuery};

use crate::state::{
    ACK_STATUS, GAS_FACTOR, LAST_OUTBOUND_NONCE, OUTBOUND_CALLS_STATE, OWNER,
    TEMP_STATE_CREATE_OUTBOUND_REPLY_ID,
};

pub fn forwarder_query(deps: Deps<RouterQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        QueryMsg::FetchOwner {} => to_binary(&fetch_owner(deps)?),
        QueryMsg::FetchCustodyContract { chain_id } => {
            to_binary(&fetch_custody_contract(deps, &chain_id)?)
        }
        QueryMsg::FetchAllCustodyContracts {} => to_binary(&fetch_all_custody_contracts(deps)?),
        QueryMsg::FetchAckData { nonce } => to_binary(&fetch_ack_data(deps, nonce)?),
        QueryMsg::FetchContractCalls { nonce } => to_binary(&fetch_contract_calls(deps, nonce)?),
        QueryMsg::FetchTempItem {} => to_binary(&fetch_temp_state(deps)?),
        QueryMsg::FetchRecentOutboundNonce {} => to_binary(&fetch_recent_out_bound_nonce(deps)?),
        QueryMsg::FetchGasFactor {} => to_binary(&fetch_gas_factor(deps)?),
        QueryMsg::FetchGasPrice { chain_id } => to_binary(&fetch_oracle_gas_price(deps, chain_id)?),
        QueryMsg::FetchGasLimit {} => to_binary(&fetch_gas_limit(deps)?),
        QueryMsg::FetchDeployer {} => to_binary(&fetch_deployer(deps)?),
        QueryMsg::FetchChainType { chain_id } => to_binary(&fetch_chain_type(deps, &chain_id)?),
    }
}

/**
 * @notice Used to fetch the owner address
*/
pub fn fetch_owner(deps: Deps<RouterQuery>) -> StdResult<String> {
    OWNER.load(deps.storage)
}

/**
 * @notice Used to fetch the custody contract
 * @param   chain_id
*/
pub fn fetch_custody_contract(deps: Deps<RouterQuery>, chain_id: &str) -> StdResult<String> {
    CUSTODY_CONTRACT_MAPPING.load(deps.storage, chain_id)
}

/**
    @notice Used to fetch if all white listed contracts details
*/
pub fn fetch_all_custody_contracts(deps: Deps<RouterQuery>) -> StdResult<Vec<(String, String)>> {
    match CUSTODY_CONTRACT_MAPPING
        .range(deps.storage, None, None, Order::Ascending)
        .collect()
    {
        Ok(data) => return Ok(data),
        Err(err) => return Err(err),
    };
}

pub fn fetch_ack_data(deps: Deps<RouterQuery>, nonce: u64) -> StdResult<String> {
    let ack_status_key: String = nonce.to_string();
    ACK_STATUS.load(deps.storage, &ack_status_key)
}

pub fn fetch_contract_calls(deps: Deps<RouterQuery>, nonce: u64) -> StdResult<RouterMsg> {
    let ack_status_key: String = nonce.to_string();
    OUTBOUND_CALLS_STATE.load(deps.storage, &ack_status_key)
}

pub fn fetch_temp_state(deps: Deps<RouterQuery>) -> StdResult<Vec<RouterMsg>> {
    TEMP_STATE_CREATE_OUTBOUND_REPLY_ID.load(deps.storage)
}

pub fn fetch_recent_out_bound_nonce(deps: Deps<RouterQuery>) -> StdResult<u64> {
    LAST_OUTBOUND_NONCE.load(deps.storage)
}

/**
 * @notice Used to fetch gas factor value
*/
pub fn fetch_gas_factor(deps: Deps<RouterQuery>) -> StdResult<u64> {
    GAS_FACTOR.load(deps.storage)
}

/**
 * @notice Used to fetch gas factor value
*/
pub fn fetch_gas_limit(deps: Deps<RouterQuery>) -> StdResult<u64> {
    GAS_LIMIT.load(deps.storage)
}

/**
 * @notice Used to fetch gas factor value
*/
pub fn fetch_deployer(deps: Deps<RouterQuery>) -> StdResult<String> {
    Ok(DEPLOYER.load(deps.storage).unwrap().to_string())
}

/**
 * @notice Used to fetch chain_info.
 * @param   chain_id
*/
pub fn fetch_chain_type(deps: Deps<RouterQuery>, chain_id: &str) -> StdResult<u64> {
    CHAIN_TYPE_MAPPING.load(deps.storage, chain_id)
}
