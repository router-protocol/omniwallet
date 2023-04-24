use crate::state::{
    CREATE_OUTBOUND_REPLY_ID, LAST_OUTBOUND_NONCE, OUTBOUND_CALLS_STATE,
    TEMP_STATE_CREATE_OUTBOUND_REPLY_ID,
};
use cosmwasm_std::{from_binary, Reply, StdError, SubMsgResult};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{DepsMut, Env, Response, StdResult};
use router_wasm_bindings::types::CrosschainRequestResponse;
use router_wasm_bindings::{RouterMsg, RouterQuery};

pub fn handle_reply(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    msg: Reply,
) -> StdResult<Response<RouterMsg>> {
    match msg.id {
        CREATE_OUTBOUND_REPLY_ID => {
            let info_str: String = format!("msg_id {:?}, msg_result: {:?}", msg.id, msg.result);
            deps.api.debug(&info_str);
            // TODO: need to handle nonce data here, logic depends on the msg binary data structure.
            let mut response: Response<RouterMsg> = Response::new();
            match msg.result {
                SubMsgResult::Ok(msg_result) => match msg_result.data {
                    Some(binary_data) => {
                        deps.api.debug("Binary Data Found");
                        deps.api.debug(&binary_data.to_string());
                        let outbound_responses: CrosschainRequestResponse =
                            from_binary(&binary_data).unwrap();

                        let cross_talk_nonce: u64 = outbound_responses.request_identifier;
                        let mut temp_state_vec: Vec<RouterMsg> =
                            TEMP_STATE_CREATE_OUTBOUND_REPLY_ID.load(deps.storage)?;
                        deps.api.debug("Handling the nonce info");

                        LAST_OUTBOUND_NONCE.save(deps.storage, &cross_talk_nonce)?;
                        let temp_state: &RouterMsg = &temp_state_vec[0];
                        let ack_status_key: String = cross_talk_nonce.to_string();
                        deps.api.debug(&ack_status_key);

                        OUTBOUND_CALLS_STATE.save(deps.storage, &ack_status_key, temp_state)?;
                        let mut att_key = String::from("outbound_nonce_");
                        att_key.push_str(&cross_talk_nonce.to_string());
                        response = response.add_attribute(att_key, cross_talk_nonce.to_string());
                        temp_state_vec.remove(0);
                        TEMP_STATE_CREATE_OUTBOUND_REPLY_ID.save(deps.storage, &temp_state_vec)?;
                        return Ok(response);
                    }
                    None => deps.api.debug("No Binary Data Found"),
                },
                SubMsgResult::Err(err) => deps.api.debug(&err.to_string()),
            }
            return Ok(response);
        }
        id => return Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}
