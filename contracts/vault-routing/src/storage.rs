#![allow(dead_code)]

use soroban_sdk::{Env, Vec};

use crate::types::{DataKey, RouteRecord, RoutingConfig};

pub fn get_config(env: &Env) -> Option<RoutingConfig> {
    env.storage().instance().get(&DataKey::Config)
}

pub fn set_config(env: &Env, config: &RoutingConfig) {
    env.storage().instance().set(&DataKey::Config, config);
}

pub fn get_route_ids(env: &Env) -> Vec<u64> {
    env.storage()
        .persistent()
        .get(&DataKey::RouteIds)
        .unwrap_or_else(|| Vec::new(env))
}

pub fn set_route_ids(env: &Env, ids: &Vec<u64>) {
    env.storage().persistent().set(&DataKey::RouteIds, ids);
}

pub fn get_route(env: &Env, route_id: u64) -> Option<RouteRecord> {
    env.storage().persistent().get(&DataKey::Route(route_id))
}

pub fn set_route(env: &Env, route: &RouteRecord) {
    env.storage()
        .persistent()
        .set(&DataKey::Route(route.route_id), route);
}
