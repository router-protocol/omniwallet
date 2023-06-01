use cosmwasm_std::{Addr, Deps, MessageInfo, StdError, StdResult};
use router_wasm_bindings::RouterQuery;

use crate::state::{DEPLOYER, FORWARDER_DEPLOYER, OWNER};

pub fn is_owner_modifier(deps: Deps<RouterQuery>, info: &MessageInfo) -> StdResult<()> {
    let owner: String = match OWNER.load(deps.storage) {
        Ok(owner) => owner,
        Err(err) => return StdResult::Err(err),
    };
    if owner != info.sender.to_string() {
        return StdResult::Err(StdError::GenericErr {
            msg: String::from("Auth: Invalid Owner"),
        });
    }
    Ok(())
}

pub fn is_deployer_modifier(deps: Deps<RouterQuery>, info: &MessageInfo) -> StdResult<()> {
    let deployer: Addr = match DEPLOYER.load(deps.storage) {
        Ok(owner) => owner,
        Err(err) => return StdResult::Err(err),
    };
    if deployer != info.sender {
        return StdResult::Err(StdError::GenericErr {
            msg: String::from("Auth: Invalid Owner"),
        });
    }
    Ok(())
}

pub fn is_forwarder_deployer_modifier(
    deps: Deps<RouterQuery>,
    info: &MessageInfo,
) -> StdResult<()> {
    let forwarder_deployer: Addr = match FORWARDER_DEPLOYER.load(deps.storage) {
        Ok(forwarder_deployer) => forwarder_deployer,
        Err(err) => return StdResult::Err(err),
    };
    if forwarder_deployer != info.sender {
        return StdResult::Err(StdError::GenericErr {
            msg: String::from("Auth: Invalid Forwarder Deployer"),
        });
    }
    Ok(())
}

pub fn is_owner_or_forwarder_contract(
    deps: Deps<RouterQuery>,
    info: &MessageInfo,
) -> StdResult<()> {
    if is_owner_modifier(deps, info).is_ok() {
        return Ok(());
    }
    is_forwarder_deployer_modifier(deps, info)
}
