use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;
use crate::msg::Royalties;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub flagged: bool,
    pub chat_id: String, // Assuming chat_id is of type i32
    pub sender_address: String,
    pub receiver_address: String,
    pub owner: String,
    pub messages: Vec<MessageState>, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Eq)]
pub struct MessageState {
    pub id: String,
    pub owner: String,
    pub message_type: String,
    pub message: String,
}

pub const STATE: Item<State> = Item::new("state");