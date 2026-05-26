use soroban_sdk::{contracttype, symbol_short, Address, Env, Symbol, Vec};

use crate::types::AllocationRoute;

const ADMIN: Symbol = symbol_short!("admin");
const PAUSED: Symbol = symbol_short!("paused");
const ROUTES: Symbol = symbol_short!("routes");

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Route(Symbol),
}

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

pub fn get_route_ids(env: &Env) -> Vec<Symbol> {
    env.storage()
        .instance()
        .get(&ROUTES)
        .unwrap_or_else(|| Vec::new(env))
}

pub fn set_route_ids(env: &Env, route_ids: &Vec<Symbol>) {
    env.storage().instance().set(&ROUTES, route_ids);
}

pub fn set_route(env: &Env, route: &AllocationRoute) {
    env.storage()
        .persistent()
        .set(&DataKey::Route(route.route_id.clone()), route);
}

pub fn get_route(env: &Env, route_id: Symbol) -> Option<AllocationRoute> {
    env.storage().persistent().get(&DataKey::Route(route_id))
}
