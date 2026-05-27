#![allow(dead_code)]

use soroban_sdk::{Env, Vec};

use crate::types::{DataKey, RoundFinalizerConfig, RoundRecord};

pub fn get_config(env: &Env) -> Option<RoundFinalizerConfig> {
    env.storage().instance().get(&DataKey::Config)
}

pub fn set_config(env: &Env, config: &RoundFinalizerConfig) {
    env.storage().instance().set(&DataKey::Config, config);
}

pub fn get_round_ids(env: &Env) -> Vec<u64> {
    env.storage()
        .persistent()
        .get(&DataKey::RoundIds)
        .unwrap_or_else(|| Vec::new(env))
}

pub fn set_round_ids(env: &Env, ids: &Vec<u64>) {
    env.storage().persistent().set(&DataKey::RoundIds, ids);
}

pub fn get_round(env: &Env, round_id: u64) -> Option<RoundRecord> {
    env.storage().persistent().get(&DataKey::Round(round_id))
}

pub fn set_round(env: &Env, round: &RoundRecord) {
    env.storage()
        .persistent()
        .set(&DataKey::Round(round.round_id), round);
}
