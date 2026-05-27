#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

mod storage;
mod test;
mod types;

use storage::*;
use types::*;

#[contract]
pub struct VaultRoutingContract;

fn utilization_bps(used: u32, capacity: u32) -> u32 {
    if capacity == 0 {
        return 0;
    }
    let capped_used = if used > capacity { capacity } else { used };
    ((capped_used as u128 * 10_000u128) / capacity as u128) as u32
}

#[contractimpl]
impl VaultRoutingContract {
    pub fn initialize(env: Env, admin: Address) {
        if get_config(&env).is_some() {
            panic!("already initialized");
        }
        set_config(
            &env,
            &RoutingConfig {
                admin,
                paused: false,
            },
        );
    }

    pub fn set_paused(env: Env, admin: Address, paused: bool) {
        let mut cfg = get_config(&env).unwrap_or_else(|| panic!("not configured"));
        admin.require_auth();
        if admin != cfg.admin {
            panic!("not admin");
        }
        cfg.paused = paused;
        set_config(&env, &cfg);
    }

    pub fn upsert_route(
        env: Env,
        admin: Address,
        route_id: u64,
        capacity_units: u32,
        used_units: u32,
        failover_target_configured: bool,
    ) {
        let cfg = get_config(&env).unwrap_or_else(|| panic!("not configured"));
        admin.require_auth();
        if admin != cfg.admin {
            panic!("not admin");
        }

        if get_route(&env, route_id).is_none() {
            let mut ids = get_route_ids(&env);
            ids.push_back(route_id);
            set_route_ids(&env, &ids);
        }

        set_route(
            &env,
            &RouteRecord {
                route_id,
                capacity_units,
                used_units,
                failover_target_configured,
            },
        );
    }

    pub fn get_route_saturation_summary(env: Env) -> RouteSaturationSummary {
        let Some(cfg) = get_config(&env) else {
            return RouteSaturationSummary {
                status: RoutingStatus::Unconfigured,
                total_routes: 0,
                saturated_routes: 0,
                average_utilization_bps: 0,
                max_utilization_bps: 0,
            };
        };

        let ids = get_route_ids(&env);
        let mut saturated_routes = 0u32;
        let mut total_utilization = 0u64;
        let mut max_utilization_bps = 0u32;

        for route_id in ids.iter() {
            if let Some(route) = get_route(&env, route_id) {
                let util = utilization_bps(route.used_units, route.capacity_units);
                total_utilization = total_utilization.saturating_add(util as u64);
                if util >= 9_500 {
                    saturated_routes += 1;
                }
                if util > max_utilization_bps {
                    max_utilization_bps = util;
                }
            }
        }

        let total_routes = ids.len();
        let average_utilization_bps = if total_routes == 0 {
            0
        } else {
            (total_utilization / total_routes as u64) as u32
        };

        RouteSaturationSummary {
            status: if cfg.paused {
                RoutingStatus::Paused
            } else {
                RoutingStatus::Active
            },
            total_routes,
            saturated_routes,
            average_utilization_bps,
            max_utilization_bps,
        }
    }

    pub fn get_failover_readiness(env: Env, route_id: u64) -> FailoverReadiness {
        let Some(cfg) = get_config(&env) else {
            return FailoverReadiness {
                status: RoutingStatus::Unconfigured,
                route_id,
                is_ready: false,
                utilization_bps: 0,
                missing_failover_target: true,
            };
        };

        let Some(route) = get_route(&env, route_id) else {
            return FailoverReadiness {
                status: if cfg.paused {
                    RoutingStatus::Paused
                } else {
                    RoutingStatus::Active
                },
                route_id,
                is_ready: false,
                utilization_bps: 0,
                missing_failover_target: true,
            };
        };

        let util = utilization_bps(route.used_units, route.capacity_units);
        let missing_failover_target = !route.failover_target_configured;
        let is_ready = !cfg.paused && !missing_failover_target && util < 9_500;

        FailoverReadiness {
            status: if cfg.paused {
                RoutingStatus::Paused
            } else {
                RoutingStatus::Active
            },
            route_id,
            is_ready,
            utilization_bps: util,
            missing_failover_target,
        }
    }
}
