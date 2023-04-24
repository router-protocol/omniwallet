#[cfg(not(feature = "library"))]
use cosmwasm_std::{Binary, DepsMut, Env, Response, StdResult};
use router_wasm_bindings::{RouterMsg, RouterQuery};

pub fn handle_sudo_request(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    request_sender: String,
    src_chain_id: String,
    request_identifier: u64,
    _payload: Binary,
) -> StdResult<Response<RouterMsg>> {
    let info_str: String = format!(
        "sender: {:?}, src_chain_id: {:?}, event_nonce: {:?}",
        request_sender, src_chain_id, request_identifier
    );
    deps.api.debug(&info_str);
    deps.api
        .debug("Currently, We are not handling any inbound request, Please try again later.");
    return Ok(Response::new());
}
