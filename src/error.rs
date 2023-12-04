use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Unauthorized: sender_address = {sender_address}, receiver_address = {receiver_address}, owner={owner}")]
    Unauthorized {
        sender_address: String,
        receiver_address: String,
        owner: String,
    },

    #[error("Not found")]
    NotFound {}
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}