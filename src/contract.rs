#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point};
use cosmwasm_std::{to_binary, Binary, DepsMut, Deps, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, Sapient};
use crate::state::{State, config, config_read};


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State { 
  owner: info.sender.clone(),
  planet_name: msg.planet_name,
  planet_sapients: msg.planet_sapients,
};
config(deps.storage).save(&state)?;
Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
   match msg {
         ExecuteMsg::SetPlanetName { to } => set_planet_name(to, deps, env, info),
        ExecuteMsg::SetSapientNames { to } => set_sapient_names(to, deps, env, info),
    }
}


pub fn set_planet_name(
    to: String,
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
   // let mut state = query_config(deps);
   let mut state = config(deps.storage).load()?;
  if info.sender != state.owner {
    return Err(ContractError::Unauthorized {});
  }
//   let state = State { 
//   planet_name: to,
// };
state.planet_name = to.clone();
config(deps.storage).save(&state)?;
Ok(Response::new()
.add_attribute("action", "set_planet_name")
.add_attribute("names", to.clone()))

}


pub fn set_sapient_names(
    to: Vec<Sapient>, 
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
  // let mut state = query_config(deps);
   let mut state = config(deps.storage).load()?;
  if info.sender != state.owner {
    return Err(ContractError::Unauthorized {});
  }
//   let state = State { 
//   planet_sapients: to,
// };
state.planet_sapients = to;
config(deps.storage).save(&state)?;
Ok(Response::new()
.add_attribute("action", "set_sapient_names"))
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> { 
  match msg {
    QueryMsg::Config {} => to_binary(&query_config(deps)?),
    QueryMsg::Acknowledge { cyberdized } => to_binary(&cyberdized)
  }
}

fn query_config(deps: Deps) -> StdResult<State> {
    let state = config_read(deps.storage).load()?;
    Ok(state)
}
