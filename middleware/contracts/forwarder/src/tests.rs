use std::marker::PhantomData;

use crate::contract::execute;
use crate::contract::instantiate;
use crate::msg::{CustodyContractInfo, ExecuteMsg, InstantiateMsg, TransferInfo};
use crate::state::DEFAULT_EXPIRY_CONFIG;
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    DepsMut,
};
use cosmwasm_std::{CosmosMsg, OwnedDeps, Uint128};
use router_wasm_bindings::types::OutboundBatchRequest;
use router_wasm_bindings::{RouterMsg, RouterQuery};

const INIT_ADDRESS: &str = "init_address";

fn get_mock_dependencies() -> OwnedDeps<MockStorage, MockApi, MockQuerier, RouterQuery> {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockQuerier::default(),
        custom_query_type: PhantomData,
    }
}

fn do_instantiate(mut deps: DepsMut<RouterQuery>) {
    let instantiate_msg = InstantiateMsg {};
    let info = mock_info(INIT_ADDRESS, &[]);
    let env = mock_env();
    let res = instantiate(deps.branch(), env, info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn test_execute_update_bridge_address() {
    let mut deps = get_mock_dependencies();
    do_instantiate(deps.as_mut());
    let env = mock_env();
    let exp_timestamp: u64 = env.block.time.seconds() + DEFAULT_EXPIRY_CONFIG;

    let msg: ExecuteMsg = ExecuteMsg::SetCustodyContracts {
        custody_contracts: vec![CustodyContractInfo {
            chain_id: String::from("43113"),
            chain_type: 0,
            address: String::from("0xFEE2d12E90b721df5C135F25E2B4Ae9E483CcE3D"),
        }],
    };
    let info = mock_info(INIT_ADDRESS, &[]);
    assert_eq!(
        execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok(),
        true
    );

    let msg: ExecuteMsg = ExecuteMsg::InitiateTransfer {
        chain_id: String::from("43113"),
        chain_type: 0,
        transfers: vec![TransferInfo {
            token_address: String::from(""),
            recipient: String::from("0xFEE2d12E90b721df5C135F25E2B4Ae9E483CcE3D"),
            amount: Uint128::from(123u128),
            is_native: true,
        }],
        is_atomic: None,
    };
    let info = mock_info(INIT_ADDRESS, &[]);
    let response = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(response.messages.len(), 1);

    let message = response.messages.get(0).unwrap();
    let router_msg = message.msg.clone();
    match router_msg {
        CosmosMsg::Custom(msg) => match msg {
            RouterMsg::OutboundBatchRequests {
                outbound_batch_requests,
            } => {
                assert_eq!(outbound_batch_requests.len(), 1);
                let request: OutboundBatchRequest = outbound_batch_requests[0].clone();
                assert_eq!(request.destination_chain_id, "43113");
                let contract: Vec<u8> = request.contract_calls[0]
                    .destination_contract_address
                    .clone();
                let mut contract_string = String::from("0x");
                contract_string.push_str(&hex::encode(contract));
                assert_eq!(
                    contract_string,
                    String::from("0xfee2d12e90b721df5c135f25e2b4ae9e483cce3d")
                );
                assert_eq!(request.exp_timestamp, exp_timestamp);
            }
        },
        _ => {}
    }
}
