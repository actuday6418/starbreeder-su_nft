#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{DSOResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:superuser";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        dso_name: msg.dso_name.clone(),
        dso_coordinate: msg.dso_coordinate,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("dso_name", msg.dso_name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TransferOwner { new_owner } => try_transfer(deps, info, new_owner),
        ExecuteMsg::VetoStarbreeder { msg } => try_veto(deps, info, msg),
    }
}

pub fn try_transfer(deps: DepsMut, info: MessageInfo, new_owner: Addr) -> Result<Response, ContractError> {
  
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.owner = new_owner;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "transfer ownership"))
}

pub fn try_veto(deps: DepsMut, info: MessageInfo, _msg: String) -> Result<Response, ContractError> {
  
  //This is placeholder. Intention is to make whatever priveleged action the superuser has over here.
    STATE.update(deps.storage, |state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "veto"))
  //placeholder
  //
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetDSO {} => to_binary(&query_superuser_info(deps)?),
    }
}

fn query_superuser_info(deps: Deps) -> StdResult<DSOResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(DSOResponse { name: state.dso_name, coordinate: state.dso_coordinate })
}
