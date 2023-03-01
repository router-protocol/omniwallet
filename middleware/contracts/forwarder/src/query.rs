use crate::{
    msg::QueryMsg,
    state::{CUSTODY_CONTRACT_MAPPING, GAS_LIMIT},
    utils::fetch_oracle_gas_price,
};
use cosmwasm_std::{to_binary, Addr, Binary, Deps, Env, Order, StdResult};
use cw2::get_contract_version;
use router_wasm_bindings::{types::OutboundBatchRequest, RouterQuery};

use crate::state::{
    ACK_STATUS, GAS_FACTOR, LAST_OUTBOUND_NONCE, OUTBOUND_CALLS_STATE, OWNER,
    TEMP_STATE_CREATE_OUTBOUND_REPLY_ID,
};

pub fn forwarder_query(deps: Deps<RouterQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        QueryMsg::FetchOwner {} => to_binary(&fetch_owner(deps)?),
        QueryMsg::FetchCustodyContract {
            chain_id,
            chain_type,
        } => to_binary(&fetch_custody_contract(deps, chain_id, chain_type)?),
        QueryMsg::FetchAllCustodyContracts {} => to_binary(&fetch_all_custody_contracts(deps)?),
        QueryMsg::FetchAckData {
            destination_chain_id,
            destination_chain_type,
            outbound_batch_nonce,
        } => to_binary(&fetch_ack_data(
            deps,
            destination_chain_id,
            destination_chain_type,
            outbound_batch_nonce,
        )?),
        QueryMsg::FetchContractCalls {
            destination_chain_id,
            destination_chain_type,
            outbound_batch_nonce,
        } => to_binary(&fetch_contract_calls(
            deps,
            destination_chain_id,
            destination_chain_type,
            outbound_batch_nonce,
        )?),
        QueryMsg::FetchTempItem {} => to_binary(&fetch_temp_state(deps)?),
        QueryMsg::FetchRecentOutboundNonce {} => to_binary(&fetch_recent_out_bound_nonce(deps)?),
        QueryMsg::FetchGasFactor {} => to_binary(&fetch_gas_factor(deps)?),
        QueryMsg::FetchGasPrice {
            chain_id,
            chain_type,
        } => to_binary(&fetch_oracle_gas_price(deps, chain_id, chain_type)?),
        QueryMsg::FetchGasLimit {} => to_binary(&fetch_gas_limit(deps)?),
    }
}

/**
 * @notice Used to fetch the owner address
*/
pub fn fetch_owner(deps: Deps<RouterQuery>) -> StdResult<Addr> {
    OWNER.load(deps.storage)
}

/**
 * @notice Used to fetch the custody contract
 * @param   chain_id
 * @param   chain_type
*/
pub fn fetch_custody_contract(
    deps: Deps<RouterQuery>,
    chain_id: String,
    chain_type: u32,
) -> StdResult<String> {
    CUSTODY_CONTRACT_MAPPING.load(deps.storage, (chain_id, chain_type))
}

/**
    @notice Used to fetch if all white listed contracts details
*/
pub fn fetch_all_custody_contracts(
    deps: Deps<RouterQuery>,
) -> StdResult<Vec<((String, u32), String)>> {
    match CUSTODY_CONTRACT_MAPPING
        .range(deps.storage, None, None, Order::Ascending)
        .collect()
    {
        Ok(data) => return Ok(data),
        Err(err) => return Err(err),
    };
}

pub fn fetch_ack_data(
    deps: Deps<RouterQuery>,
    destination_chain_id: String,
    destination_chain_type: u64,
    outbound_batch_nonce: u64,
) -> StdResult<String> {
    let mut ack_status_key: String = destination_chain_id.clone();
    ack_status_key.push_str(&destination_chain_type.to_string());
    ack_status_key.push_str(&outbound_batch_nonce.to_string());

    ACK_STATUS.load(deps.storage, &ack_status_key)
}

pub fn fetch_contract_calls(
    deps: Deps<RouterQuery>,
    destination_chain_id: String,
    destination_chain_type: u64,
    outbound_batch_nonce: u64,
) -> StdResult<OutboundBatchRequest> {
    let mut ack_status_key: String = destination_chain_id.clone();
    ack_status_key.push_str(&destination_chain_type.to_string());
    ack_status_key.push_str(&outbound_batch_nonce.to_string());

    OUTBOUND_CALLS_STATE.load(deps.storage, &ack_status_key)
}

pub fn fetch_temp_state(deps: Deps<RouterQuery>) -> StdResult<Vec<OutboundBatchRequest>> {
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
