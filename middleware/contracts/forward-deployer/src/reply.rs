use crate::state::{FORWARDER_CONTRACT_MAPPING, INSTANTIATE_REPLY_ID, TEMP_FORWARDER_OWNER};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{DepsMut, Env, Response, StdResult};
use cosmwasm_std::{Reply, StdError};
use omni_wallet::parse::parse_reply_instantiate_data;

use router_wasm_bindings::{RouterMsg, RouterQuery};

pub fn handle_reply(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    msg: Reply,
) -> StdResult<Response<RouterMsg>> {
    match msg.id {
        INSTANTIATE_REPLY_ID => {
            let response = parse_reply_instantiate_data(msg).unwrap();
            let forwarder_contract: String = response.contract_address;

            let forwarder_owner: String = TEMP_FORWARDER_OWNER.load(deps.storage)?;
            FORWARDER_CONTRACT_MAPPING.save(
                deps.storage,
                forwarder_owner.clone(),
                &forwarder_contract,
            )?;
            return Ok(Response::new().add_attribute("forwarder_owner", forwarder_owner));
        }
        id => return Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}
