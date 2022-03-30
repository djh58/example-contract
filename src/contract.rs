#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use sha2::{Sha256, Digest};

use crate::error::ContractError;
use crate::msg::{NumberResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:example-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        number: 0,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("number", state.number.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Random {} => try_pseudorandom(deps, _env, _info),
    }
}

pub fn try_pseudorandom(deps: DepsMut, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let mut hasher = Sha256::new();
    hasher.update(_env.block.height.to_string().as_bytes());
    let result = hasher.finalize()[0];


    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.number = result;
        Ok(state)
    })?;

    Ok(Response::new()
        .add_attribute("method", "try_pseudorandom")
        .add_attribute("number", result.to_string()))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetNumber {} => to_binary(&query_number(deps)?),
    }
}

fn query_number(deps: Deps) -> StdResult<NumberResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(NumberResponse { number: state.number })
}

