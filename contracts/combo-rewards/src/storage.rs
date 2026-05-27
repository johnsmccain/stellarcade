#![allow(dead_code)]

use soroban_sdk::{Address, Env};

use crate::types::{ComboRewardsConfig, DataKey, StreakComboRecord};

pub fn get_config(env: &Env) -> Option<ComboRewardsConfig> {
    env.storage().instance().get(&DataKey::Config)
}

pub fn set_config(env: &Env, cfg: &ComboRewardsConfig) {
    env.storage().instance().set(&DataKey::Config, cfg);
}

pub fn get_player(env: &Env, player: &Address) -> Option<StreakComboRecord> {
    env.storage().persistent().get(&DataKey::Player(player.clone()))
}

pub fn set_player(env: &Env, record: &StreakComboRecord) {
    env.storage()
        .persistent()
        .set(&DataKey::Player(record.player.clone()), record);
}
