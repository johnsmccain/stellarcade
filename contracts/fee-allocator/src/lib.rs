#![no_std]

mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec};

pub use types::{AllocationDriftSummary, AllocationRoute, RebalanceReadiness, RouteDrift};

const BPS_DENOMINATOR: u32 = 10_000;
const DEFAULT_DRIFT_THRESHOLD: i128 = 1;

#[contract]
pub struct FeeAllocator;

#[contractimpl]
impl FeeAllocator {
    pub fn init(env: Env, admin: Address) {
        if storage::get_admin(&env).is_none() {
            storage::set_admin(&env, &admin);
            storage::set_paused(&env, false);
            storage::set_route_ids(&env, &Vec::new(&env));
        }
    }

    pub fn set_paused(env: Env, admin: Address, paused: bool) {
        require_admin(&env, &admin);
        storage::set_paused(&env, paused);
    }

    pub fn upsert_route(
        env: Env,
        admin: Address,
        route_id: Symbol,
        target_bps: u32,
        allocated_amount: i128,
    ) {
        require_admin(&env, &admin);
        assert!(target_bps <= BPS_DENOMINATOR, "target bps too high");
        assert!(allocated_amount >= 0, "allocation must be non-negative");

        if storage::get_route(&env, route_id.clone()).is_none() {
            let mut route_ids = storage::get_route_ids(&env);
            route_ids.push_back(route_id.clone());
            storage::set_route_ids(&env, &route_ids);
        }

        storage::set_route(
            &env,
            &AllocationRoute {
                route_id,
                target_bps,
                allocated_amount,
            },
        );
    }

    /// Returns a structured drift summary for every allocation route.
    ///
    /// Expected amounts use floor division:
    /// `(total_allocated * target_bps) / 10000`. Missing and empty states
    /// return an empty route list and zero totals.
    pub fn allocation_drift_summary(env: Env) -> AllocationDriftSummary {
        let configured = storage::is_configured(&env);
        let paused = storage::is_paused(&env);
        let routes = load_routes(&env);

        let mut total_allocated = 0i128;
        let mut target_bps_total = 0u32;
        for route in routes.iter() {
            total_allocated = total_allocated.saturating_add(route.allocated_amount);
            target_bps_total = target_bps_total.saturating_add(route.target_bps);
        }

        let mut route_drifts = Vec::new(&env);
        let mut total_drift = 0i128;
        let mut max_route_drift = 0i128;

        for route in routes.iter() {
            let expected_amount = total_allocated
                .saturating_mul(route.target_bps as i128)
                / BPS_DENOMINATOR as i128;
            let drift_amount = abs_diff(route.allocated_amount, expected_amount);
            total_drift = total_drift.saturating_add(drift_amount);
            if drift_amount > max_route_drift {
                max_route_drift = drift_amount;
            }
            route_drifts.push_back(RouteDrift {
                route_id: route.route_id,
                target_bps: route.target_bps,
                allocated_amount: route.allocated_amount,
                expected_amount,
                drift_amount,
            });
        }

        let target_bps_valid = target_bps_total == BPS_DENOMINATOR;
        let balanced = target_bps_valid && total_drift <= DEFAULT_DRIFT_THRESHOLD;

        AllocationDriftSummary {
            configured,
            paused,
            route_count: route_drifts.len(),
            total_allocated,
            target_bps_total,
            total_drift,
            max_route_drift,
            drift_threshold: DEFAULT_DRIFT_THRESHOLD,
            target_bps_valid,
            balanced,
            routes: route_drifts,
        }
    }

    /// Returns whether current allocations can be rebalanced.
    ///
    /// Rebalancing is ready only when the allocator is configured, unpaused,
    /// target bps sum to 10000, at least one route has funds, and drift is
    /// greater than the rounding threshold.
    pub fn rebalance_readiness(env: Env) -> RebalanceReadiness {
        let summary = Self::allocation_drift_summary(env);
        let has_drift = summary.total_drift > summary.drift_threshold;
        let ready_to_rebalance = summary.configured
            && !summary.paused
            && summary.route_count > 0
            && summary.total_allocated > 0
            && summary.target_bps_valid
            && has_drift;

        RebalanceReadiness {
            configured: summary.configured,
            paused: summary.paused,
            route_count: summary.route_count,
            total_allocated: summary.total_allocated,
            target_bps_total: summary.target_bps_total,
            target_bps_valid: summary.target_bps_valid,
            drift_threshold: summary.drift_threshold,
            total_drift: summary.total_drift,
            max_route_drift: summary.max_route_drift,
            has_drift,
            ready_to_rebalance,
        }
    }
}

fn require_admin(env: &Env, admin: &Address) {
    admin.require_auth();
    let stored = storage::get_admin(env).expect("not initialized");
    assert!(stored == *admin, "unauthorized");
}

fn load_routes(env: &Env) -> Vec<AllocationRoute> {
    let route_ids = storage::get_route_ids(env);
    let mut routes = Vec::new(env);
    for route_id in route_ids.iter() {
        if let Some(route) = storage::get_route(env, route_id) {
            routes.push_back(route);
        }
    }
    routes
}

fn abs_diff(left: i128, right: i128) -> i128 {
    if left >= right {
        left - right
    } else {
        right - left
    }
}

#[cfg(test)]
mod test;
