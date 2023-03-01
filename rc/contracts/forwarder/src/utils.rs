use cosmwasm_std::{Deps, StdResult};
use router_wasm_bindings::{types::GasPriceResponse, RouterQuerier, RouterQuery};

use crate::state::GAS_FACTOR;

pub fn fetch_oracle_gas_price(
    deps: Deps<RouterQuery>,
    chain_id: String,
    chain_type: u32,
) -> StdResult<u64> {
    let router_querier: RouterQuerier = RouterQuerier::new(&deps.querier);
    // TODO: we can have some default value in case of query error
    let gas_price_response: GasPriceResponse = router_querier.gas_price(chain_id, chain_type)?;
    let gas_factor: u64 = GAS_FACTOR.load(deps.storage).unwrap_or(110);
    Ok((gas_price_response.gas_price * gas_factor) / 100)
}
