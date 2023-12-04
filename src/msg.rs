use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    pub flagged: bool,
    pub chat_id: String,
    pub sender_address: String,
    pub receiver_address: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Royalties {
    pub seller_fee_basis_points: u32,
    pub creators: Vec<Creator>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OwnerOf {
    pub message_id: String,
} 

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetApprovals {
    message_id: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MintingInfo {

}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
// pub struct ownerOfWrapper {
//     pub owner_of: OwnerOf
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Creator {
    pub address: String,
    pub share: i8
}

#[cw_serde]
pub enum ExecuteMsg {
    Flag { enabled: bool },
    SendMessage {
        id : String,
        message_type:String,
        message:String
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Tmessage {
    pub transfer_nft: SendTokenMsg
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Rmessage {
    pub revoke: Revoke
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct SendTokenMsg {
    pub recipient: String,
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Revoke {
    pub spender: String,
    pub message_id: String
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetMetadataResponse)]
    GetMetadata {},

    #[returns(GetMessageResponse)]
    GetMessages {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetMetadataResponse { 
    pub flagged: bool,
    pub chat_id: String,
    pub sender_address: String,
    pub receiver_address: String,
    pub owner: String,
  
}

#[cw_serde]
pub struct GetMessageResponse {
    pub number: i32,
    pub messages: Vec<MessageState>
}

#[cw_serde]
pub struct MessageState {
    pub id: String,
    pub owner: String,
    pub message_type: String,
    pub message: String,
}
