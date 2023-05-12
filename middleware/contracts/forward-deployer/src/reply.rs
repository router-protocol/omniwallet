use crate::state::{
    DEPLOYER, FORWARDER_CONTRACT_MAPPING, INSTANTIATE_REPLY_ID, TEMP_DEPLOY_CONTRACT,
    TEMP_FORWARDER_OWNER,
};
use cosmwasm_std::{to_binary, CosmosMsg, Reply, StdError, WasmMsg};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{DepsMut, Env, Response, StdResult};
use omni_wallet::{forwarder_deployer::DeployerExecuteMsg, parse::parse_reply_instantiate_data};

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
            let deployer: String = DEPLOYER.load(deps.storage)?;
            let (code, salt, constructor_args, chain_ids, gas_limits, gas_prices) =
                TEMP_DEPLOY_CONTRACT.load(deps.storage)?;
            let exec_msg: CosmosMsg<RouterMsg> = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: deployer,
                funds: vec![],
                msg: to_binary(&DeployerExecuteMsg::DeployContract {
                    code,
                    salt,
                    constructor_args,
                    chain_ids,
                    gas_limits,
                    gas_prices,
                    forwarder_contract,
                })?,
            });
            return Ok(Response::new()
                .add_message(exec_msg)
                .add_attribute("forwarder_owner", forwarder_owner));
        }
        id => return Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}
