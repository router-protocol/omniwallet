use crate::{
    execution::_set_chain_types_info,
    state::{
        CHAIN_TYPE_MAPPING, DEFAULT_GAS_LIMIT, DEPLOYER, FORWARDER_DEPLOYER, GAS_FACTOR, GAS_LIMIT,
    },
};
use omni_wallet::forwarder::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use router_wasm_bindings::{types::ChainType, RouterMsg, RouterQuery, SudoMsg};

#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{Reply, StdError};
use cw2::set_contract_version;

use crate::{
    execution::forwarder_execute, handle_acknowledgement::handle_sudo_ack,
    handle_inbound::handle_sudo_request, handle_reply::handle_reply, query::forwarder_query,
    state::OWNER,
};

// version info for migration info
const CONTRACT_NAME: &str = "forwarder-contract";
const CONTRACT_VERSION: &str = "0.1.06";

#[cfg_attr(not(feature = "library"), entry_point)]
/// Instantiates the smart contract.
pub fn instantiate(
    mut deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<RouterMsg>> {
    deps.api.debug("Instantiating the contract🚀");

    // Save the owner address
    OWNER.save(deps.storage, &msg.owner)?;

    // Save the deployer address
    FORWARDER_DEPLOYER.save(deps.storage, &info.sender)?;

    // Save the deployer address with validation
    DEPLOYER.save(deps.storage, &deps.api.addr_validate(&msg.deployer)?)?;

    // Save the gas factor
    GAS_FACTOR.save(deps.storage, &110)?;

    // Save the gas limit
    GAS_LIMIT.save(deps.storage, &DEFAULT_GAS_LIMIT)?;

    // Save the chain type mapping
    CHAIN_TYPE_MAPPING.save(
        deps.storage,
        &env.block.chain_id,
        &ChainType::ChainTypeCosmos.get_chain_code(),
    )?;

    // Set chain type information
    let response: Response<RouterMsg> = _set_chain_types_info(deps.branch(), msg.chain_type_info)?;

    // Set the contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(response.add_attribute("action", "forwarder-contract-init"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<RouterQuery>, env: Env, msg: SudoMsg) -> StdResult<Response<RouterMsg>> {
    match msg {
        SudoMsg::HandleIReceive {
            request_sender,
            src_chain_id,
            request_identifier,
            payload,
        } => handle_sudo_request(
            deps,
            env,
            request_sender,
            src_chain_id,
            request_identifier,
            payload,
        ),
        SudoMsg::HandleIAck {
            request_identifier,
            exec_flag,
            exec_data,
            refund_amount,
        } => handle_sudo_ack(
            deps,
            env,
            request_identifier,
            exec_flag,
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
