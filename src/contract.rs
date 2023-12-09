#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, BankMsg,Decimal};
use cw2::set_contract_version;


use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, 
    GetMetadataResponse, 
    InstantiateMsg, 
    QueryMsg, 

    OwnerOf,

};
use crate::state::{State, STATE, MessageState};


use serde::{Deserialize, Serialize};

// version info for migration info
const CONTRACT_NAME: &str = "NINJAKITS Messaging";
const CONTRACT_VERSION: &str = "0.0.1";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GetOwnerResponse {
    pub owner: String,
    pub approvals: Vec<Approval>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Approval {
    spender: String
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let state = State {
        flagged: false,
        chat_id: msg.chat_id,
        sender_address: msg.sender_address.clone(),
        receiver_address: msg.receiver_address,
        messages: vec![],
        owner: msg.sender_address.clone(), 
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendMessage { id,message_type,message } => execute::send_message(deps,&info, id,message_type,message,info.sender.clone()),
        ExecuteMsg::Flag { enabled } => execute::flag(enabled, deps,info.sender),
    }
}

pub mod execute {
    use cosmwasm_std::{Uint128, coins, WasmMsg};

    #[allow(unused_imports)]
    use crate::state;

    use super::*;

    pub enum Messages {
        Execute(WasmMsg),
        Bank(BankMsg)
    }

    pub fn flag(enabled: bool, deps: DepsMut,owner: Addr) -> Result<Response, ContractError> {
        let s = STATE.load(deps.storage)?;
        if owner.as_str() !=  &s.sender_address {
            if owner.as_str() !=  &s.receiver_address {
                return Err(ContractError::Unauthorized {
                    sender_address: s.sender_address.clone(),
                    receiver_address: s.receiver_address.clone(),
                    owner:owner.to_string()
                });
            }
        }
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            
            state.flagged = enabled;
            Ok(state)
        }).unwrap();

        Ok(Response::new())
    }


    #[derive(Debug, Serialize, Deserialize)]
    struct QueryWrapper {
        pub owner_of: OwnerOf
    }

    pub fn send_message(deps: DepsMut,info: &MessageInfo, id: String,message_type:String,message:String,owner: Addr) -> Result<Response, ContractError> {
        let s = STATE.load(deps.storage)?;
        let mut resp:Response = Response::new();
        if owner.as_str() !=  &s.sender_address {
            if owner.as_str() !=  &s.receiver_address {
                return Err(ContractError::Unauthorized {
                    sender_address: s.sender_address.clone(),
                    receiver_address: s.receiver_address.clone(),
                    owner:owner.to_string()
                });
            }
        }

    
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
                // If the token with the given id doesn't exist, add a new token to the `messages` vector
                state.messages.push(MessageState {
                    id: id.to_string(),
                    owner: owner.to_string(),
                    message_type: message_type.to_string(),
                    message: message.to_string(),
                });

            Ok(state)
        })?;
        let payment: Uint128 = cw_utils::must_pay(info, "inj").unwrap();
        if payment != Uint128::from(100000000000000u128) { // need to rework this to include platform fee and royalties
            return Err(ContractError::InsufficientFunds {});
        } else {
            resp = resp
                .add_message(BankMsg::Send {  
                    to_address: "inj1gc48836cwjt26y2cx95f8y4wwnjrt3tvwhl2rq".into(),
                    amount: coins(100000000000000, "inj"),
                });
        }
    
        Ok(Response::new().add_attribute("action", "increment"))
    }



}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMetadata {} => to_json_binary(&query::get_metadata(deps)?),
        QueryMsg::GetMessages {} => to_json_binary(&query::get_messages(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn get_metadata(deps: Deps) -> StdResult<GetMetadataResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetMetadataResponse {
            flagged: state.flagged,
            chat_id: state.chat_id,
            sender_address: state.sender_address,
            receiver_address: state.receiver_address,
            owner: state.owner, 
        })
    }

    pub fn get_messages(deps: Deps) -> StdResult<Vec<MessageState>> {
        let state = STATE.load(deps.storage)?;
        Ok(state.messages)
    }

}

