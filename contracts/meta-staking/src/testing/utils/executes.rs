use anyhow::Result as AnyResult;
use cosmwasm_std::{coin, Addr, Uint128};
use cw_multi_test::{App, AppResponse, Executor};

use mesh_apis::{StakingExecuteMsg as ExecuteMsg, StakingSudoMsg};

use mesh_testing::{constants::NATIVE_DENOM, macros::addr};

pub fn delegate(
    app: &mut App,
    contract_addr: &str,
    sender: &str,
    validator: &str,
    amount: Uint128,
) -> AnyResult<AppResponse> {
    app.execute_contract(
        addr!(sender),
        addr!(contract_addr),
        &ExecuteMsg::Delegate {
            validator: validator.to_string(),
            amount,
        },
        &[],
    )
}

pub fn undelegate(
    app: &mut App,
    contract_addr: &str,
    sender: &str,
    validator: &str,
    amount: Uint128,
) -> AnyResult<AppResponse> {
    app.execute_contract(
        addr!(sender),
        addr!(contract_addr),
        &ExecuteMsg::Undelegate {
            validator: validator.to_string(),
            amount,
        },
        &[],
    )
}

pub fn withdraw_rewards(
    app: &mut App,
    contract_addr: &str,
    sender: &str,
    validator: &str,
) -> AnyResult<AppResponse> {
    app.execute_contract(
        addr!(sender),
        addr!(contract_addr),
        &ExecuteMsg::WithdrawDelegatorReward {
            validator: validator.to_string(),
        },
        &[],
    )
}

pub fn add_consumer(
    app: &mut App,
    contract_addr: &str,
    sender: &str,
    consumer_addr: &str,
    funds_avaiable: u128,
) -> AnyResult<AppResponse> {
    let sudo_msg = StakingSudoMsg::AddConsumer {
        consumer_address: consumer_addr.to_string(),
        funds_available_for_staking: coin(funds_avaiable, NATIVE_DENOM),
    };

    app.execute_contract(
        addr!(sender),
        addr!(contract_addr),
        &ExecuteMsg::Sudo(sudo_msg),
        &[],
    )
}

pub fn remove_consumer(
    app: &mut App,
    contract_addr: &str,
    sender: &str,
    consumer_addr: &str,
) -> AnyResult<AppResponse> {
    let sudo_msg = StakingSudoMsg::RemoveConsumer {
        consumer_address: consumer_addr.to_string(),
    };

    app.execute_contract(
        addr!(sender),
        addr!(contract_addr),
        &ExecuteMsg::Sudo(sudo_msg),
        &[],
    )
}

// TODO: withdraw to consumer end with IBC call which is not supported by cw-multi-test
pub fn _withdraw_to_consumer(
    _app: &mut App,
    _contract_addr: Addr,
    _sender: &str,
    _consumer: &str,
    _validator: &str,
) -> AnyResult<AppResponse> {
    unimplemented!()
    // app.execute_contract(
    //     addr!(sender),
    //     contract_addr.clone(),
    //     &ExecuteMsg::WithdrawToCostumer { consumer: consumer.to_string(), validator: validator.to_string() },
    //     &[],
    // )
}
