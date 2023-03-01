use crate::state::ACK_STATUS;
use cosmwasm_std::{Binary, Coin, DepsMut, Env, Response, StdResult};
use router_wasm_bindings::{Bytes, RouterMsg, RouterQuery};

pub fn handle_out_bound_ack_request(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    sender: String,
    destination_chain_type: u32,
    destination_chain_id: String,
    outbound_batch_nonce: u64,
    execution_code: u64,
    execution_status: bool,
    exec_flags: Vec<bool>,
    exec_data: Vec<Binary>,
    refund_amount: Coin,
) -> StdResult<Response<RouterMsg>> {
    let chain_type: u32 = destination_chain_type;
    let mut ack_status_key: String = destination_chain_id.clone();
    ack_status_key.push_str(&chain_type.to_string());
    ack_status_key.push_str(&outbound_batch_nonce.to_string());

    let mut data: Vec<Bytes> = vec![];
    for i in 0..exec_data.len() {
        data.push(exec_data[i].0.clone());
    }
    let info_str: String = format!(
        "handle_out_bound_ack_request-- destination_chain_type: {}, destination_chain_id: {}, sender: {}, outbound_batch_nonce: {}, execution_code {:?}, execution_status {:?}, exec_flags {:?}, exec_data {:?}, refund_amount {:?}",
        chain_type, destination_chain_id, sender.clone(), outbound_batch_nonce, execution_code, execution_status, exec_flags, data, refund_amount
    );
    ACK_STATUS.save(deps.storage, &ack_status_key, &info_str)?;
    deps.api.debug(&info_str);

    let response = Response::new()
        .add_attribute("sender", sender)
        .add_attribute("destination_chain_type", chain_type.to_string())
        .add_attribute("destination_chain_id", destination_chain_id.clone())
        .add_attribute("outbound_batch_nonce", outbound_batch_nonce.to_string());

    Ok(response)
}
