use soroban_sdk::{symbol_short, Address, Env, Symbol};

use crate::types::LootPool;

const ADMIN: Symbol = symbol_short!("admin");
const PAUSED: Symbol = symbol_short!("paused");
const POOL: Symbol = symbol_short!("pool");

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&ADMIN, admin);
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&ADMIN)
}

pub fn is_configured(env: &Env) -> bool {
    env.storage().instance().has(&ADMIN)
}

pub fn set_paused(env: &Env, paused: bool) {
    env.storage().instance().set(&PAUSED, &paused);
}

pub fn is_paused(env: &Env) -> bool {
    env.storage().instance().get(&PAUSED).unwrap_or(false)
}

pub fn set_pool(env: &Env, pool: &LootPool) {
    env.storage().instance().set(&POOL, pool);
}

pub fn get_pool(env: &Env) -> Option<LootPool> {
    env.storage().instance().get(&POOL)
}
