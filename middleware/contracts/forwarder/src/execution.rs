use cosmwasm_std::{
    Coin, DepsMut, Env, MessageInfo, ReplyOn, Response, StdError, StdResult, SubMsg, Uint128,
};
use router_wasm_bindings::{
    ethabi::{
        encode,
        ethereum_types::{H160, U256},
        Token,
    },
    types::{ContractCall, OutboundBatchRequest, OutgoingTxFee},
    utils::convert_address_from_string_to_bytes,
    Bytes, RouterMsg, RouterQuery,
};

use crate::{
    modifers::{is_deployer_modifier, is_owner_modifier},
    msg::{CustodyContractInfo, ExecuteMsg, TransferInfo},
    state::{
        CREATE_OUTBOUND_REPLY_ID, CUSTODY_CONTRACT_MAPPING, DEFAULT_EXPIRY_CONFIG, DEPLOYER,
        GAS_FACTOR, GAS_LIMIT, OWNER, TEMP_STATE_CREATE_OUTBOUND_REPLY_ID,
    },
    utils::fetch_oracle_gas_price,
};

pub fn forwarder_execute(
    deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    match msg {
        ExecuteMsg::InitiateTransfer {
            chain_id,
            chain_type,
            transfers,
            is_atomic,
        } => init_transfers(
            deps, &env, &info, chain_id, chain_type, transfers, is_atomic,
        ),
        ExecuteMsg::SetCustodyContracts { custody_contracts } => {
            set_custody_contracts(deps, &env, &info, custody_contracts)
        }
        ExecuteMsg::SetGasLimit { limit } => set_gas_limit(deps, &env, &info, limit),
        ExecuteMsg::SetGasFactor { gas_factor } => set_gas_factor(deps, &env, &info, gas_factor),
        ExecuteMsg::SetOwner { new_owner } => set_owner(deps, &env, &info, new_owner),
        ExecuteMsg::SetDeployer { deployer } => set_deployer(deps, &env, &info, deployer),
    }
}

/**
 * @notice Used to initiate transfers call on the destination chain.
 * @notice Only callable by admin.
 * @param   chain_id    destination chain id
 * @param   chain_type  destination chain type
 * @param   transfers   transfers info
 * @param   is_atomic   is transfers atomic, default value will be true
*/
pub fn init_transfers(
    deps: DepsMut<RouterQuery>,
    env: &Env,
    info: &MessageInfo,
    chain_id: String,
    chain_type: u32,
    transfers: Vec<TransferInfo>,
    is_atomic: Option<bool>,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), info)?;

    let mut contract_calls: Vec<ContractCall> = vec![];

    let custody_contract: String =
        CUSTODY_CONTRACT_MAPPING.load(deps.storage, (chain_id.clone(), chain_type))?;
    let byte_address: Bytes = convert_address_from_string_to_bytes(custody_contract, chain_type)?;
    for i in 0..transfers.len() {
        // paylaod {address(to_address), address(token), uint256(amount), bool(isNative)}
        let recipient_address: Bytes =
            convert_address_from_string_to_bytes(transfers[i].recipient.clone(), chain_type)?;
        let address_h160 = H160::from_slice(&recipient_address);
        let recipient_address_token: Token = Token::Address(address_h160);

        let token_address: Bytes =
            convert_address_from_string_to_bytes(transfers[i].token_address.clone(), chain_type)?;
        let address_h160: H160 = H160::from_slice(&token_address);
        let token_address_token: Token = Token::Address(address_h160);

        let u256: U256 = U256::from(transfers[i].amount.u128());
        let amount_token: Token = Token::Uint(u256);
        let is_native_token: Token = Token::Bool(transfers[i].is_native);
        let contract_call_payload: Bytes = encode(&[
            recipient_address_token,
            token_address_token,
            amount_token,
            is_native_token,
        ]);
        let contract_call: ContractCall = ContractCall {
            destination_contract_address: byte_address.clone(),
            payload: contract_call_payload,
        };
        contract_calls.push(contract_call);
    }

    let expiry_timeout: u64 = DEFAULT_EXPIRY_CONFIG;
    let outbound_batch_req: OutboundBatchRequest = OutboundBatchRequest {
        destination_chain_type: chain_type,
        destination_chain_id: chain_id.clone(),
        contract_calls,
        relayer_fee: Coin {
            denom: String::from("route"),
            amount: Uint128::zero(),
        },
        outgoing_tx_fee: OutgoingTxFee {
            gas_limit: GAS_LIMIT.load(deps.storage)?,
            gas_price: fetch_oracle_gas_price(deps.as_ref(), chain_id.clone(), chain_type)?,
        },
        is_atomic: is_atomic.unwrap_or(true),
        exp_timestamp: env.block.time.seconds() + expiry_timeout,
    };
    let outbound_batch_requests: Vec<OutboundBatchRequest> = vec![outbound_batch_req];
    TEMP_STATE_CREATE_OUTBOUND_REPLY_ID.save(deps.storage, &outbound_batch_requests)?;
    let outbound_batch_reqs: RouterMsg = RouterMsg::OutboundBatchRequests {
        outbound_batch_requests,
    };

    let outbound_submessage: SubMsg<RouterMsg> = SubMsg {
        gas_limit: None,
        id: CREATE_OUTBOUND_REPLY_ID,
        reply_on: ReplyOn::Success,
        msg: outbound_batch_reqs.into(),
    };
    let res = Response::new()
        .add_submessage(outbound_submessage)
        .add_attribute("action", "SetCustodyContracts");
    Ok(res)
}

/**
 * @notice Used to add custody contracts info.
 * @notice Only callable by admin.
 * @param   custody_contracts  list of custody contracts info
*/
pub fn set_custody_contracts(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    custody_contracts: Vec<CustodyContractInfo>,
) -> StdResult<Response<RouterMsg>> {
    is_deployer_modifier(deps.as_ref(), info)?;

    for i in 0..custody_contracts.len() {
        // let address: String = custody_contracts[i].address.replace("0x", "").to_lowercase();
        CUSTODY_CONTRACT_MAPPING.save(
            deps.storage,
            (
                custody_contracts[i].chain_id.clone(),
                custody_contracts[i].chain_type,
            ),
            &custody_contracts[i].address,
        )?;
    }

    let res = Response::new().add_attribute("action", "SetCustodyContracts");
    Ok(res)
}

/**
 * @notice Used to set gas factor
 * @notice Only callable by Admin.
 * @param  gas_factor gas factor value
*/
pub fn set_gas_factor(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    gas_factor: u64,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), &info)?;
    if gas_factor < 100 {
        return Err(StdError::GenericErr {
            msg: String::from("GasFactor: Can not be less than 100%"),
        });
    }
    if gas_factor > 1000 {
        return Err(StdError::GenericErr {
            msg: String::from("GasFactor: Can not be greater than 1000%"),
        });
    }

    GAS_FACTOR.save(deps.storage, &gas_factor)?;

    let res = Response::new().add_attribute("action", "SetGasFactor");
    Ok(res)
}

/**
 * @notice Used to set gas limit
 * @notice Only callable by Admin.
 * @param  limit gas limit value
*/
pub fn set_gas_limit(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    gas_limit: u64,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), &info)?;
    GAS_LIMIT.save(deps.storage, &gas_limit)?;

    let res = Response::new().add_attribute("action", "SetGasLimit");
    Ok(res)
}

/**
 * @notice Used to set new owner
 * @notice Only callable by Admin.
 * @param  new_owner
*/
pub fn set_owner(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    new_owner: String,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), &info)?;

    OWNER.save(deps.storage, &deps.api.addr_validate(&new_owner)?)?;

    let res = Response::new().add_attribute("action", "SetOwner");
    Ok(res)
}

/**
 * @notice Used to set new deployer
 * @notice Only callable by Admin.
 * @param  new_owner
*/
pub fn set_deployer(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    deployer: String,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), &info)?;

    DEPLOYER.save(deps.storage, &deps.api.addr_validate(&deployer)?)?;

    let res = Response::new().add_attribute("action", "SetDeployer");
    Ok(res)
}
