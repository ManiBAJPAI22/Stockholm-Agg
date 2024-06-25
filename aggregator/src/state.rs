use cw_storage_plus::Map;
use cosmwasm_std::Uint128;

pub const NEIGHBORS_AND_COSTS: Map<&str, Vec<(String, Uint128)>> =
    Map::new("neighbors_and_costs");