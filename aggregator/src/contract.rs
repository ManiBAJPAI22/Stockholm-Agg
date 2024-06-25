use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ShortestPathResponse};
use crate::state::NEIGHBORS_AND_COSTS;
use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

// Implement From<ContractError> for StdError
impl From<ContractError> for StdError {
    fn from(error: ContractError) -> Self {
        StdError::generic_err(error.to_string())
    }
}

fn heuristic(_source: &str, _target: &str) -> Uint128 {
    // Implement your heuristic function here
    Uint128::zero()
}

fn calculate_cost(_source: &str, _neighbor: &str, amount: Uint128) -> Uint128 {
    // Implement your cost calculation function here
    amount
}

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Initialize the contract state here
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CalculateShortestPath {
            source,
            target,
            amount,
        } => {
            let (path, _) = find_shortest_path(deps.as_ref(), &source, &target, amount)?;
            Ok(Response::new().add_attribute("path", path.join(",")))
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetShortestPath {
            source,
            target,
            amount,
        } => {
            let (path, cost) = find_shortest_path(deps, &source, &target, amount)?;
            to_json_binary(&ShortestPathResponse { path, cost })
        }
    }
}

fn find_shortest_path(
    deps: Deps,
    source: &str,
    target: &str,
    amount: Uint128,
) -> Result<(Vec<String>, Uint128), ContractError> {
    let mut open_set: BinaryHeap<Reverse<(Uint128, String)>> = BinaryHeap::new();
    open_set.push(Reverse((Uint128::zero(), source.to_string())));

    let mut g_score: HashMap<String, Uint128> = HashMap::new();
    g_score.insert(source.to_string(), Uint128::zero());

    let mut f_score: HashMap<String, Uint128> = HashMap::new();
    f_score.insert(
        source.to_string(),
        heuristic(source, target),
    );

    let mut previous: HashMap<String, Option<String>> = HashMap::new();
    previous.insert(source.to_string(), None);

    while let Some(Reverse((_, current_token))) = open_set.pop() {
        if current_token == target {
            let mut path = Vec::new();
            let mut token = Some(current_token);
            while let Some(t) = token {
                path.push(t.clone());
                token = previous.get(&t).unwrap().clone();
            }
            path.reverse();
            return Ok((path, g_score[target]));
        }

        let neighbors_and_costs = NEIGHBORS_AND_COSTS.load(deps.storage, current_token.as_str())?;
        for (neighbor, _base_cost) in neighbors_and_costs {
            let tentative_g_score = g_score[&current_token] + calculate_cost(&current_token, &neighbor, amount);
            let neighbor_g_score = *g_score.get(&neighbor).unwrap_or(&Uint128::MAX);
            if tentative_g_score < neighbor_g_score {
                previous.insert(neighbor.clone(), Some(current_token.clone()));
                g_score.insert(neighbor.clone(), tentative_g_score);
                let f_score_value = tentative_g_score + heuristic(&neighbor, target);
                f_score.insert(neighbor.clone(), f_score_value);
                open_set.push(Reverse((f_score_value, neighbor)));
            }
        }
    }

    Ok((Vec::new(), Uint128::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_json, StdResult};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn execute_calculate_shortest_path() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        let _ = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // Set up test data
        NEIGHBORS_AND_COSTS
            .save(
                deps.as_mut().storage,
                "A",
                &vec![("B".to_string(), Uint128::new(1)), ("C".to_string(), Uint128::new(4))],
            )
            .unwrap();
        NEIGHBORS_AND_COSTS
            .save(
                deps.as_mut().storage,
                "B",
                &vec![("D".to_string(), Uint128::new(1))],
            )
            .unwrap();
        NEIGHBORS_AND_COSTS
            .save(
                deps.as_mut().storage,
                "C",
                &vec![("D".to_string(), Uint128::new(1))],
            )
            .unwrap();

        let msg = ExecuteMsg::CalculateShortestPath {
            source: "A".to_string(),
            target: "D".to_string(),
            amount: Uint128::new(1),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(1, res.attributes.len());
        assert_eq!("path", res.attributes[0].key);
        assert_eq!("A,B,D", res.attributes[0].value);
    }

    #[test]
    fn query_get_shortest_path() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        let _ = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Set up test data
        NEIGHBORS_AND_COSTS
            .save(
                deps.as_mut().storage,
                "A",
                &vec![("B".to_string(), Uint128::new(1)), ("C".to_string(), Uint128::new(4))],
            )
            .unwrap();
        NEIGHBORS_AND_COSTS
            .save(
                deps.as_mut().storage,
                "B",
                &vec![("D".to_string(), Uint128::new(1))],
            )
            .unwrap();
        NEIGHBORS_AND_COSTS
            .save(
                deps.as_mut().storage,
                "C",
                &vec![("D".to_string(), Uint128::new(1))],
            )
            .unwrap();

        let msg = QueryMsg::GetShortestPath {
            source: "A".to_string(),
            target: "D".to_string(),
            amount: Uint128::new(1),
        };

        let res: StdResult<ShortestPathResponse> = from_json(&query(deps.as_ref(), mock_env(), msg).unwrap());
        let response = res.unwrap();
        assert_eq!(vec!["A", "B", "D"], response.path);
        assert_eq!(Uint128::new(2), response.cost);
    }

    #[test]
    fn test_no_path() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        let _ = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Set up test data with no path to target
        NEIGHBORS_AND_COSTS
            .save(
                deps.as_mut().storage,
                "A",
                &vec![("B".to_string(), Uint128::new(1))],
            )
            .unwrap();
        NEIGHBORS_AND_COSTS
            .save(
                deps.as_mut().storage,
                "B",
                &vec![],
            )
            .unwrap();
        NEIGHBORS_AND_COSTS
            .save(
                deps.as_mut().storage,
                "C",
                &vec![("D".to_string(), Uint128::new(1))],
            )
            .unwrap();
        NEIGHBORS_AND_COSTS
            .save(
                deps.as_mut().storage,
                "D",
                &vec![],
            )
            .unwrap();

        let msg = QueryMsg::GetShortestPath {
            source: "A".to_string(),
            target: "D".to_string(),
            amount: Uint128::new(1),
        };

        let res: StdResult<ShortestPathResponse> = from_json(&query(deps.as_ref(), mock_env(), msg).unwrap());
        let response = res.unwrap();
        assert_eq!(Vec::<String>::new(), response.path);
        assert_eq!(Uint128::MAX, response.cost);
    }
}