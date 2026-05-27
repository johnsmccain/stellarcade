#![allow(dead_code)]

use soroban_sdk::Env;

use crate::types::{BonusVaultConfig, BonusVaultState, DataKey};

pub fn get_config(env: &Env) -> Option<BonusVaultConfig> {
    env.storage().instance().get(&DataKey::Config)
}

pub fn set_config(env: &Env, cfg: &BonusVaultConfig) {
    env.storage().instance().set(&DataKey::Config, cfg);
}

pub fn get_state(env: &Env) -> Option<BonusVaultState> {
    env.storage().instance().get(&DataKey::State)
}

pub fn set_state(env: &Env, state: &BonusVaultState) {
    env.storage().instance().set(&DataKey::State, state);
}
