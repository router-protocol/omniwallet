use crate::{
    reply::handle_reply,
    state::{DEPLOYER, FORWARDER_CODE_ID},
};
use omni_wallet::forwarder_deployer::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use router_wasm_bindings::{RouterMsg, RouterQuery};

#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{Reply, StdError};
use cw2::set_contract_version;

use crate::{execution::handle_execute, query::handle_query, state::OWNER};

// version info for migration info
const CONTRACT_NAME: &str = "forwarder-deployer-contract";
const CONTRACT_VERSION: &str = "0.1.07";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    deps.api.debug("Instantiating the contract🚀");

    OWNER.save(deps.storage, &info.sender)?;
    FORWARDER_CODE_ID.save(deps.storage, &msg.code_id)?;
    DEPLOYER.save(deps.storage, &msg.deployer)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("action", "forwarder-deployer-contract-init"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    handle_execute(deps, env, info, msg)
}

/// This just stores the result for future query
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
    handle_query(deps, env, msg)
}
