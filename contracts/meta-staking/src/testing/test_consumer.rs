use cosmwasm_std::{Decimal, Uint128};

use crate::{
    testing::utils::{
        executes::{add_consumer, remove_consumer},
        queries::query_consumer,
        setup::{setup_with_consumer, setup_with_contracts},
    },
    ContractError,
};

use mesh_testing::{constants::CREATOR_ADDR, macros::assert_error};

#[test]
fn add_and_remove_consumer() {
    let (mut app, meta_staking_addr, mesh_consumer_addr) = setup_with_consumer();

    let consumer = query_consumer(
        &app,
        meta_staking_addr.as_str(),
        mesh_consumer_addr.as_str(),
    )
    .unwrap();

    assert_eq!(consumer.available_funds, Uint128::new(10000_u128));
    assert_eq!(consumer.rewards.pending, Decimal::zero());

    remove_consumer(
        &mut app,
        meta_staking_addr.as_str(),
        CREATOR_ADDR,
        mesh_consumer_addr.as_str(),
    )
    .unwrap();

    // Should return error because we didn't find it
    query_consumer(
        &app,
        meta_staking_addr.as_str(),
        mesh_consumer_addr.as_str(),
    )
    .unwrap_err();
}

#[test]
fn consumer_already_exists() {
    let (mut app, meta_staking_addr, mesh_consumer_addr) = setup_with_consumer();

    // Should failed because we already have a consumer
    let err = add_consumer(
        &mut app,
        meta_staking_addr.as_str(),
        CREATOR_ADDR,
        mesh_consumer_addr.as_str(),
        10000,
    );

    assert_error!(err, ContractError::ConsumerAlreadyExists {})
}

#[test]
fn consumer_not_enough_funds() {
    let (mut app, meta_staking_addr, mesh_consumer_addr) = setup_with_contracts();

    let err = add_consumer(
        &mut app,
        meta_staking_addr.as_str(),
        CREATOR_ADDR,
        mesh_consumer_addr.as_str(),
        999999999,
    );

    assert_error!(err, ContractError::NotEnoughFunds {})
}

#[test]
fn consumer_remove_not_exists() {
    let (mut app, meta_staking_addr, mesh_consumer_addr) = setup_with_contracts();

    let err = remove_consumer(
        &mut app,
        meta_staking_addr.as_str(),
        CREATOR_ADDR,
        mesh_consumer_addr.as_str(),
    );

    assert_error!(err, ContractError::NoConsumer {})
}
