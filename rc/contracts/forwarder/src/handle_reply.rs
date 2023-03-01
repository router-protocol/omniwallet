use crate::state::{
    CREATE_OUTBOUND_REPLY_ID, LAST_OUTBOUND_NONCE, OUTBOUND_CALLS_STATE, TEMP_INBOUND_KEY,
    TEMP_STATE_CREATE_OUTBOUND_REPLY_ID,
};
use cosmwasm_std::{from_binary, Event, Reply, StdError, SubMsgResult};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{DepsMut, Env, Response, StdResult};
use router_wasm_bindings::types::{
    OutboundBatchRequest, OutboundBatchResponse, OutboundBatchResponses,
    INBOUND_OUTBOUND_MAPPING_EVENT_NAME,
};
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
                        let outbound_responses: OutboundBatchResponses =
                            from_binary(&binary_data).unwrap();

                        let response_vec: Vec<OutboundBatchResponse> =
                            outbound_responses.outbound_batch_responses;
                        let temp_state_vec: Vec<OutboundBatchRequest> =
                            TEMP_STATE_CREATE_OUTBOUND_REPLY_ID.load(deps.storage)?;
                        deps.api.debug("Handling the nonce info");
                        let mut event: Event;
                        match TEMP_INBOUND_KEY.load(deps.storage) {
                            Ok(info) => {
                                event = Event::new(INBOUND_OUTBOUND_MAPPING_EVENT_NAME)
                                    .add_attribute("src_chain_id", info.0.to_string())
                                    .add_attribute("src_chain_type", info.1.to_string())
                                    .add_attribute("event_nonce", info.2.to_string());
                            }
                            Err(_) => event = Event::new("Dummy_info"),
                        }

                        for i in 0..response_vec.len() {
                            let nonce: u64 = response_vec[i].outbound_batch_nonce;
                            LAST_OUTBOUND_NONCE.save(deps.storage, &nonce)?;
                            let temp_state: &OutboundBatchRequest = &temp_state_vec[i];
                            let mut ack_status_key: String =
                                temp_state.destination_chain_id.clone();
                            ack_status_key.push_str(&temp_state.destination_chain_type.to_string());
                            ack_status_key.push_str(&nonce.to_string());
                            deps.api.debug(&ack_status_key);

                            OUTBOUND_CALLS_STATE.save(deps.storage, &ack_status_key, temp_state)?;
                            let mut att_key = String::from("outbound_nonce_");
                            att_key.push_str(&i.to_string());
                            response = response.add_attribute(att_key, nonce.to_string());

                            event = event.add_attribute(
                                "dest_chain_id",
                                temp_state.destination_chain_id.to_string(),
                            );
                            event = event.add_attribute(
                                "dest_chain_type",
                                temp_state.destination_chain_type.to_string(),
                            );
                            event = event.add_attribute("outbound_nonce", nonce.to_string());
                        }
                        return Ok(response.add_event(event));
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
