use std::marker::PhantomData;

use crate::contract::instantiate;
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::OwnedDeps;
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    DepsMut,
};
use omni_wallet::forwarder_contract::InstantiateMsg;
use router_wasm_bindings::RouterQuery;

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
        code_id: 1,
        deployer: INIT_ADDRESS.to_string(),
    };
    let info = mock_info(INIT_ADDRESS, &[]);
    let env = mock_env();
    let res = instantiate(deps.branch(), env, info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn test_basic() {
    let mut deps = get_mock_dependencies();
    do_instantiate(deps.as_mut());
}
