#![no_std]

mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, Address, Env};

pub use types::{ActivePoolSnapshot, LootPool, RolloverDelay};

#[contract]
pub struct LootRotation;

#[contractimpl]
impl LootRotation {
    pub fn init(env: Env, admin: Address) {
        if storage::get_admin(&env).is_none() {
            storage::set_admin(&env, &admin);
            storage::set_paused(&env, false);
        }
    }

    pub fn set_paused(env: Env, admin: Address, paused: bool) {
        require_admin(&env, &admin);
        storage::set_paused(&env, paused);
    }

    pub fn set_active_pool(
        env: Env,
        admin: Address,
        pool_id: u64,
        item_count: u32,
        reward_weight: u32,
        starts_at: u64,
        ends_at: u64,
    ) {
        require_admin(&env, &admin);
        assert!(item_count > 0, "pool must contain items");
        assert!(reward_weight > 0, "reward weight must be positive");
        assert!(ends_at >= starts_at, "ends before start");

        storage::set_pool(
            &env,
            &LootPool {
                pool_id,
                item_count,
                reward_weight,
                starts_at,
                ends_at,
            },
        );
    }

    /// Returns a stable active-pool read model for client rotation screens.
    ///
    /// Empty state returns `has_active_pool = false` with zeroed pool values.
    /// `seconds_until_rollover` uses saturating subtraction and is zero once
    /// the pool has reached or passed `ends_at`.
    pub fn active_pool_snapshot(env: Env) -> ActivePoolSnapshot {
        let now = env.ledger().timestamp();
        let configured = storage::is_configured(&env);
        let paused = storage::is_paused(&env);

        match storage::get_pool(&env) {
            Some(pool) => ActivePoolSnapshot {
                configured,
                paused,
                has_active_pool: true,
                pool_id: pool.pool_id,
                item_count: pool.item_count,
                reward_weight: pool.reward_weight,
                starts_at: pool.starts_at,
                ends_at: pool.ends_at,
                now,
                seconds_until_rollover: pool.ends_at.saturating_sub(now),
            },
            None => ActivePoolSnapshot {
                configured,
                paused,
                has_active_pool: false,
                pool_id: 0,
                item_count: 0,
                reward_weight: 0,
                starts_at: 0,
                ends_at: 0,
                now,
                seconds_until_rollover: 0,
            },
        }
    }

    /// Returns rollover timing in a compact shape for operators.
    ///
    /// Missing pools return `rollover_due = false` and a zero delay.
    pub fn rollover_delay(env: Env) -> RolloverDelay {
        let snapshot = Self::active_pool_snapshot(env);
        RolloverDelay {
            configured: snapshot.configured,
            paused: snapshot.paused,
            has_active_pool: snapshot.has_active_pool,
            rollover_due: snapshot.has_active_pool && snapshot.seconds_until_rollover == 0,
            now: snapshot.now,
            ends_at: snapshot.ends_at,
            seconds_until_rollover: snapshot.seconds_until_rollover,
        }
    }
}

fn require_admin(env: &Env, admin: &Address) {
    admin.require_auth();
    let stored = storage::get_admin(env).expect("not initialized");
    assert!(stored == *admin, "unauthorized");
}

#[cfg(test)]
mod test;
