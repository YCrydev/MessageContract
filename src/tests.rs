use std::str::FromStr;

use cosmwasm_std::{Addr, Uint128, Decimal, Empty, coins};
use cw_multi_test::{App, ContractWrapper, Executor};
use nft_multi_test::{self, cw721_contract};

use crate::{contract::*, msg::{InstantiateMsg, Creator, ExecuteMsg,QueryMsg}, ContractError};

type Extension = Option<Empty>;

#[test]
fn init() {
    let mut app = App::default();
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            flagged:false,
            chat_id: "testid".to_string(),
            sender_address: "address1".to_string(),
            receiver_address: "address2".to_string(),
            owner: "address1".to_string(), 
        }, 
        &vec![], 
        "Instantiate Exchange Contract", 
        None
    ).expect("contract failed to instantiate");
}

#[test]
fn send_message() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("address1"), coins(100000000000000, "inj"))
            .unwrap()
    });
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let chat =   app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            flagged:false,
            chat_id: "testid".to_string(),
            sender_address: "address1".to_string(),
            receiver_address: "address2".to_string(),
            owner: "address1".to_string(), 
        }, 
        &vec![], 
        "Instantiate Exchange Contract", 
        None
    ).expect("contract failed to instantiate");


    app.execute_contract(
        Addr::unchecked("address1"),
        chat,
        &ExecuteMsg::SendMessage {
            id: 0.to_string(),
            message_type: "text".to_string(),
            message: "hey".to_string(),
        },
        &coins(100000000000000, "inj"),
    ).expect("could not send message");
}

#[test]
fn flag() {
    let mut app = App::default();
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let chat =   app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            flagged:false,
            chat_id: "testid".to_string(),
            sender_address: "address1".to_string(),
            receiver_address: "address2".to_string(),
            owner: "address1".to_string(), 
        }, 
        &vec![], 
        "Instantiate Exchange Contract", 
        None
    ).expect("contract failed to instantiate");


    app.execute_contract(
        Addr::unchecked("address1"),
        chat,
        &ExecuteMsg::Flag {
            enabled:true
        },
        &vec![],
    ).expect("could not send message");
}
// #[test]
// fn get_metadata() {
//     let mut app = App::default();
//     let code = ContractWrapper::new(execute, instantiate, query);
//     let code_id = app.store_code(Box::new(code));
//     let chat =   app.instantiate_contract(
//         code_id, 
//         Addr::unchecked("owner"), 
//         &InstantiateMsg {
//             flagged:false,
//             chat_id: "testid".to_string(),
//             sender_address: "address1".to_string(),
//             receiver_address: "address2".to_string(),
//             owner: "address1".to_string(), 
//         }, 
//         &vec![], 
//         "Instantiate Exchange Contract", 
//         None
//     ).expect("contract failed to instantiate");


//     app.query(
//         Addr::unchecked("address1"),
//         chat,
//         &QueryMsg::GetMetadata{
//         },
//         &vec![],
//     ).expect("could not send message");
// }


#[test]
fn cw_math_platform_fee() {
    let payment = Uint128::new(1000000000000000000);
    let fee = Decimal::percent(3);
    let fee_amount = payment * fee;

    assert_eq!(fee_amount, Uint128::new(30000000000000000));
}

#[test]
fn cw_math_royalties() {
    let payment = Uint128::new(1069000000000000000);
    let price = Uint128::new(1000000000000000000);
    let fee = Decimal::percent(3);
    let royalties = Decimal::from_ratio(690 as u32, 10_000u32);
    let royalty = payment - price;
    let fee = price * fee;
    assert_eq!(royalty, price * royalties);
    assert_eq!(royalty, Uint128::new(69000000000000000));
    assert_eq!(fee, Uint128::new(30000000000000000));
}