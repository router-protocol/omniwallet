#[cfg(not(feature = "library"))]
use cosmwasm_std::{Binary, DepsMut, Env, Response, StdResult};
use router_wasm_bindings::{RouterMsg, RouterQuery};

pub fn handle_in_bound_request(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    sender: String,
    src_chain_type: u32,
    src_chain_id: String,
    event_nonce: u64,
    _payload: Binary,
) -> StdResult<Response<RouterMsg>> {
    let info_str: String = format!(
        "sender: {:?}, src_chain_id: {:?}, src_chain_type: {:?}, event_nonce: {:?}",
        sender, src_chain_id, src_chain_type, event_nonce
    );
    deps.api.debug(&info_str);
    deps.api
        .debug("Currently, We are not handling any inbound request, Please try again later.");
    return Ok(Response::new());
}
