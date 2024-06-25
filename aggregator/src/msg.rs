use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CalculateShortestPath {
        source: String,
        target: String,
        amount: Uint128,
    },
}

#[cw_serde]
pub enum QueryMsg {
    GetShortestPath {
        source: String,
        target: String,
        amount: Uint128,
    },
}

#[cw_serde]
pub struct ShortestPathResponse {
    pub path: Vec<String>,
    pub cost: Uint128,
}