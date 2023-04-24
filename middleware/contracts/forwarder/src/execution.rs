use cosmwasm_std::{
    DepsMut, Env, Event, MessageInfo, ReplyOn, Response, StdError, StdResult, SubMsg, Uint128,
};
use router_wasm_bindings::{
    ethabi::{
        encode,
        ethereum_types::{H160, U256},
        Token,
    },
    types::{AckType, RequestMetaData},
    utils::convert_address_from_string_to_bytes,
    Bytes, RouterMsg, RouterQuery,
};

use crate::{
    modifers::{is_deployer_modifier, is_owner_modifier},
    state::{
        CHAIN_TYPE_MAPPING, CREATE_OUTBOUND_REPLY_ID, CUSTODY_CONTRACT_MAPPING, DEPLOYER,
        GAS_FACTOR, GAS_LIMIT, OWNER, TEMP_STATE_CREATE_OUTBOUND_REPLY_ID,
    },
    utils::fetch_oracle_gas_price,
};
use omni_wallet::forwarder::{ChainTypeInfo, CustodyContractInfo, ExecuteMsg, TransferInfo};

pub fn forwarder_execute(
    deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    match msg {
        ExecuteMsg::InitiateTransfer {
            chain_id,
            transfers,
        } => init_transfers(deps, &env, &info, chain_id, transfers),
        ExecuteMsg::SetCustodyContracts { custody_contracts } => {
            set_custody_contracts(deps, &env, &info, custody_contracts)
        }
        ExecuteMsg::SetGasLimit { limit } => set_gas_limit(deps, &env, &info, limit),
        ExecuteMsg::SetGasFactor { gas_factor } => set_gas_factor(deps, &env, &info, gas_factor),
        ExecuteMsg::SetOwner { new_owner } => set_owner(deps, &env, &info, new_owner),
        ExecuteMsg::SetDeployer { deployer } => set_deployer(deps, &env, &info, deployer),
        ExecuteMsg::SetChainTypes { chain_type_info } => {
            set_chain_types_info(deps, env, info, chain_type_info)
        }
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
    _env: &Env,
    info: &MessageInfo,
    chain_id: String,
    transfers: Vec<TransferInfo>,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), info)?;

    let mut i_requests: Vec<RouterMsg> = vec![];
    let mut sub_messages: Vec<SubMsg<RouterMsg>> = vec![];

    let custody_contract: String = CUSTODY_CONTRACT_MAPPING.load(deps.storage, &chain_id)?;
    for i in 0..transfers.len() {
        // paylaod {address(to_address), address(token), uint256(amount), bool(isNative)}
        let chain_type: u64 = CHAIN_TYPE_MAPPING.load(deps.storage, &chain_id)?;
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

        let request_packet: Bytes = encode(&[
            Token::String(custody_contract.clone()),
            Token::Bytes(contract_call_payload),
        ]);
        let gas_limit: u64 = GAS_LIMIT.load(deps.storage)?;
        let gas_price: u64 =
            fetch_oracle_gas_price(deps.as_ref(), chain_id.clone()).unwrap_or(100_000_000_000);
        let request_metadata: RequestMetaData = RequestMetaData {
            dest_gas_limit: gas_limit,
            dest_gas_price: gas_price,
            ack_gas_limit: gas_limit,
            ack_gas_price: 5000_000,
            relayer_fee: Uint128::zero(),
            ack_type: AckType::AckOnBoth,
            is_read_call: false,
            asm_address: String::default(),
        };

        let i_send_request: RouterMsg = RouterMsg::CrosschainCall {
            version: 1,
            route_amount: Uint128::new(0u128),
            route_recipient: String::default(),
            dest_chain_id: chain_id.clone(),
            request_metadata: request_metadata.get_abi_encoded_bytes(),
            request_packet,
        };
        i_requests.push(i_send_request.clone());
        let sub_msg: SubMsg<RouterMsg> = SubMsg {
            gas_limit: None,
            id: CREATE_OUTBOUND_REPLY_ID,
            reply_on: ReplyOn::Success,
            msg: i_send_request.into(),
        };
        sub_messages.push(sub_msg);
    }

    TEMP_STATE_CREATE_OUTBOUND_REPLY_ID.save(deps.storage, &i_requests)?;

    let res = Response::new()
        .add_submessages(sub_messages)
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
            &custody_contracts[i].chain_id,
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

/**
 * @notice Used to set chain type info operations of the given chain (chainId, chainType).
 * @notice Only callable by Admin.
 * @param  chain_type_info   chain infos (chain_id & chain_type)

*/
pub fn set_chain_types_info(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    info: MessageInfo,
    chain_type_info: Vec<ChainTypeInfo>,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), &info)?;

    for i in 0..chain_type_info.len() {
        CHAIN_TYPE_MAPPING.save(
            deps.storage,
            &chain_type_info[i].chain_id,
            &chain_type_info[i].chain_type,
        )?;
    }
    let event_name: String = String::from("SetChainTypeInfo");
    let set_chain_bytes_info_event: Event = Event::new(event_name);

    let res = Response::new()
        .add_attribute("action", "SetChainTypeInfo")
        .add_event(set_chain_bytes_info_event);
    Ok(res)
}
