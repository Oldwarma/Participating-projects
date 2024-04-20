use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;

use crate::msg::ProviderInfo;

#[cw_serde]
pub struct Config {
    pub provider: ProviderInfo,
    pub remote_to_local_exchange_rate: Decimal,
    pub meta_staking_contract_address: Addr,
    pub ics20_channel: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PACKET_LIFETIME: Item<u64> = Item::new("packet_time");
pub const CHANNEL: Item<String> = Item::new("channel");
