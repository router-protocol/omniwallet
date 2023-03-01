use cosmwasm_std::{Addr, Deps, MessageInfo, StdError, StdResult};
use router_wasm_bindings::RouterQuery;

use crate::state::{DEPLOYER, OWNER};

pub fn is_owner_modifier(deps: Deps<RouterQuery>, info: &MessageInfo) -> StdResult<()> {
    let owner: Addr = match OWNER.load(deps.storage) {
        Ok(owner) => owner,
        Err(err) => return StdResult::Err(err),
    };
    if owner != info.sender {
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
