use cosmwasm_std::{
    to_binary, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn, Response, StdResult, SubMsg, WasmMsg,
};
use omni_wallet::forwarder::{ChainTypeInfo, ExecuteMsg as ForwarderExecuteMsg, InstantiateMsg};
use router_wasm_bindings::{RouterMsg, RouterQuery};

use crate::{
    modifers::is_owner_modifier,
    state::{
        DEPLOYER, FORWARDER_CODE_ID, FORWARDER_CONTRACT_MAPPING, INSTANTIATE_REPLY_ID, OWNER,
        TEMP_FORWARDER_OWNER,
    },
};
use omni_wallet::forwarder_deployer::ExecuteMsg;

pub fn handle_execute(
    deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    match msg {
        ExecuteMsg::DeployForwarderContract { chain_type_info } => {
            deploy_forwarder_contract(deps, &env, &info, chain_type_info)
        }
        ExecuteMsg::SetCodeId { code_id } => set_code_id(deps, &env, &info, code_id),
        ExecuteMsg::SetForwarderAdmin { admin } => set_forwarder_admin(deps, &env, &info, admin),
        ExecuteMsg::SetOwner { new_owner } => set_owner(deps, &env, &info, new_owner),
        ExecuteMsg::SetDeployer { deployer } => set_deployer(deps, &env, &info, deployer),
    }
}

/**
 * @notice Used to deploy forwarder contract and set mapping.
*/
pub fn deploy_forwarder_contract(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    chain_type_info: Vec<ChainTypeInfo>,
) -> StdResult<Response<RouterMsg>> {
    let sender: String = info.sender.to_string();
    TEMP_FORWARDER_OWNER.save(deps.storage, &sender)?;
    assert_eq!(FORWARDER_CONTRACT_MAPPING.has(deps.storage, sender), false);

    let code_id: u64 = FORWARDER_CODE_ID.load(deps.storage)?;
    let deployer_address: String = DEPLOYER.load(deps.storage)?;

    let res = Response::new().add_attribute("action", "DeployForwarderContract");
    let res = res.add_submessage(SubMsg {
        id: INSTANTIATE_REPLY_ID,
        gas_limit: None,
        msg: WasmMsg::Instantiate {
            code_id,
            funds: vec![],
            admin: Some(info.sender.to_string()),
            label: "Forwarder Contract".to_string(),
            msg: to_binary(&InstantiateMsg {
                deployer: deployer_address,
                owner: info.sender.to_string(),
                chain_type_info,
            })?,
        }
        .into(),
        reply_on: ReplyOn::Success,
    });
    return Ok(res);
}

/**
 * @notice Used to set code id
 * @notice Only callable by admin.
 * @param   code_id
*/
pub fn set_code_id(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    code_id: u64,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), info)?;

    FORWARDER_CODE_ID.save(deps.storage, &code_id)?;

    let res = Response::new().add_attribute("action", "SetCodeId");
    Ok(res)
}

/**
 * @notice Used to set gas factor
 * @notice Only callable by Admin.
 * @param  gas_factor gas factor value
*/
pub fn set_forwarder_admin(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    admin: String,
) -> StdResult<Response<RouterMsg>> {
    let sender: String = info.sender.to_string();

    let forwarder_contract: String = FORWARDER_CONTRACT_MAPPING.load(deps.storage, sender)?;

    let exec_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: forwarder_contract,
        funds: vec![],
        msg: to_binary(&ForwarderExecuteMsg::SetOwner { new_owner: admin })?,
    });

    let res = Response::new().add_attribute("action", "SetForwarderAdmin");
    Ok(res.add_message(exec_msg))
}

/**
 * @notice Used to set new owner
 * @notice Only callable by Admin.
 * @param  new_owner
*/
pub fn set_owner(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    new_owner: String,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), &info)?;

    OWNER.save(deps.storage, &deps.api.addr_validate(&new_owner)?)?;

    let res = Response::new().add_attribute("action", "SetOwner");
    Ok(res)
}

/**
 * @notice Used to set new deployer
 * @notice Only callable by Admin.
 * @param  new_owner
*/
pub fn set_deployer(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    deployer: String,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), &info)?;
    deps.api.addr_validate(&deployer)?;

    DEPLOYER.save(deps.storage, &deployer)?;

    let res = Response::new().add_attribute("action", "SetDeployer");
    Ok(res)
}
