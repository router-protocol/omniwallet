use crate::{
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    state::{DEFAULT_GAS_LIMIT, GAS_FACTOR, GAS_LIMIT},
};
use router_wasm_bindings::{RouterMsg, RouterQuery, SudoMsg};

#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{Reply, StdError};
use cw2::set_contract_version;

use crate::{
    execution::forwarder_execute, handle_acknowledgement::handle_out_bound_ack_request,
    handle_inbound::handle_in_bound_request, handle_reply::handle_reply, query::forwarder_query,
    state::OWNER,
};

// version info for migration info
const CONTRACT_NAME: &str = "voyager-routing-contract";
const CONTRACT_VERSION: &str = "0.1.03";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    deps.api.debug("Instantiating the contract🚀");

    OWNER.save(deps.storage, &info.sender)?;
    GAS_FACTOR.save(deps.storage, &110)?;
    GAS_LIMIT.save(deps.storage, &DEFAULT_GAS_LIMIT)?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("action", "voyager-fee-manager-init"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<RouterQuery>, env: Env, msg: SudoMsg) -> StdResult<Response<RouterMsg>> {
    match msg {
        SudoMsg::HandleInboundReq {
            sender,
            chain_type,
            source_chain_id,
            event_nonce,
            payload,
        } => handle_in_bound_request(
            deps,
            env,
            sender,
            chain_type,
            source_chain_id,
            event_nonce,
            payload,
        ),
        SudoMsg::HandleOutboundAck {
            outbound_tx_requested_by,
            destination_chain_type,
            destination_chain_id,
            outbound_batch_nonce,
            execution_code,
            execution_status,
            exec_flags,
            exec_data,
            refund_amount,
        } => handle_out_bound_ack_request(
            deps,
            env,
            outbound_tx_requested_by,
            destination_chain_type,
            destination_chain_id,
            outbound_batch_nonce,
            execution_code,
            execution_status,
            exec_flags,
            exec_data,
            refund_amount,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    forwarder_execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut<RouterQuery>, env: Env, msg: Reply) -> StdResult<Response<RouterMsg>> {
    handle_reply(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut<RouterQuery>, env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME.to_string() {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    // note: better to do proper semver compare, but string compare *usually* works
    if ver.version >= CONTRACT_VERSION.to_string() {
        return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    }

    let info_str: String = format!(
        "migrating contract: {}, new_contract_version: {}, contract_name: {}",
        env.contract.address,
        CONTRACT_VERSION.to_string(),
        CONTRACT_NAME.to_string()
    );
    deps.api.debug(&info_str);
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<RouterQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    forwarder_query(deps, env, msg)
}
