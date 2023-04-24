use std::marker::PhantomData;

use crate::contract::execute;
use crate::contract::instantiate;
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    DepsMut,
};
use cosmwasm_std::{CosmosMsg, OwnedDeps, Uint128};
use omni_wallet::forwarder::ChainTypeInfo;
use omni_wallet::forwarder::{CustodyContractInfo, ExecuteMsg, InstantiateMsg, TransferInfo};
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
    let instantiate_msg = InstantiateMsg {
        deployer: INIT_ADDRESS.to_string(),
    };
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

    let msg: ExecuteMsg = ExecuteMsg::SetChainTypes {
        chain_type_info: vec![ChainTypeInfo {
            chain_id: "43113".into(),
            chain_type: 1,
        }],
    };
    let info = mock_info(INIT_ADDRESS, &[]);
    assert_eq!(
        execute(deps.as_mut(), env.clone(), info.clone(), msg).is_ok(),
        true
    );

    let msg: ExecuteMsg = ExecuteMsg::InitiateTransfer {
        chain_id: String::from("43113"),
        transfers: vec![TransferInfo {
            token_address: String::from("0x1212121212121212121212121212121212121212"),
            recipient: String::from("0xFEE2d12E90b721df5C135F25E2B4Ae9E483CcE3D"),
            amount: Uint128::from(123u128),
            is_native: true,
        }],
    };
    let info = mock_info(INIT_ADDRESS, &[]);
    let response = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(response.messages.len(), 1);

    let message = response.messages.get(0).unwrap();
    let router_msg = message.msg.clone();
    match router_msg {
        CosmosMsg::Custom(msg) => match msg {
            RouterMsg::CrosschainCall {
                version,
                route_amount,
                route_recipient,
                dest_chain_id,
                request_metadata,
                request_packet,
            } => {
                assert_eq!(version, 1);
                assert_eq!(dest_chain_id, "43113");
                assert_eq!(route_amount, Uint128::zero());
                assert_eq!(route_recipient, "");
                assert_eq!(hex::encode(request_metadata), "00000000000493e0000000174876e80000000000000493e000000000004c4b40000000000000000000000000000000000300");
                assert_eq!(hex::encode(request_packet), "000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000002a307846454532643132453930623732316466354331333546323545324234416539453438334363453344000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000fee2d12e90b721df5c135f25e2b4ae9e483cce3d0000000000000000000000001212121212121212121212121212121212121212000000000000000000000000000000000000000000000000000000000000007b0000000000000000000000000000000000000000000000000000000000000001");
            }
        },
        _ => {}
    }
}
