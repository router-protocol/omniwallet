use crate::state::ACK_STATUS;
use cosmwasm_std::{Binary, Coin, DepsMut, Env, Response, StdResult};
use router_wasm_bindings::{RouterMsg, RouterQuery};

pub fn handle_sudo_ack(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    request_identifier: u64,
    exec_flag: bool,
    exec_data: Binary,
    refund_amount: Coin,
) -> StdResult<Response<RouterMsg>> {
    let ack_status_key: String = request_identifier.to_string();

    let info_str: String = format!(
        "handle_out_bound_ack_request-- request_identifier: {}, exec_flag: {}, exec_data: {}, exec_data {:?}, refund_amount {:?}",
        request_identifier, exec_flag, exec_data.clone(), exec_data, refund_amount
    );
    ACK_STATUS.save(deps.storage, &ack_status_key, &info_str)?;
    deps.api.debug(&info_str);

    let response =
        Response::new().add_attribute("outbound_batch_nonce", request_identifier.to_string());

    Ok(response)
}
