#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

mod storage;
mod test;
mod types;

use storage::*;
use types::*;

#[contract]
pub struct ComboRewardsContract;

#[contractimpl]
impl ComboRewardsContract {
    pub fn initialize(env: Env, admin: Address) {
        if get_config(&env).is_some() {
            panic!("already initialized");
        }
        set_config(
            &env,
            &ComboRewardsConfig {
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

    pub fn upsert_player_snapshot(
        env: Env,
        admin: Address,
        player: Address,
        streak_count: u32,
        combo_multiplier_bps: u32,
        expires_at_ledger: u32,
    ) {
        let cfg = get_config(&env).unwrap_or_else(|| panic!("not configured"));
        admin.require_auth();
        if admin != cfg.admin {
            panic!("not admin");
        }

        set_player(
            &env,
            &StreakComboRecord {
                player,
                streak_count,
                combo_multiplier_bps,
                expires_at_ledger,
            },
        );
    }

    pub fn get_streak_combo_snapshot(env: Env, player: Address) -> StreakComboSnapshot {
        let Some(cfg) = get_config(&env) else {
            return StreakComboSnapshot {
                status: ComboRewardsStatus::Unconfigured,
                player,
                streak_count: 0,
                combo_multiplier_bps: 0,
                expires_at_ledger: 0,
                has_snapshot: false,
            };
        };

        let Some(record) = get_player(&env, &player) else {
            return StreakComboSnapshot {
                status: if cfg.paused {
                    ComboRewardsStatus::Paused
                } else {
                    ComboRewardsStatus::Active
                },
                player,
                streak_count: 0,
                combo_multiplier_bps: 0,
                expires_at_ledger: 0,
                has_snapshot: false,
            };
        };

        StreakComboSnapshot {
            status: if cfg.paused {
                ComboRewardsStatus::Paused
            } else {
                ComboRewardsStatus::Active
            },
            player: record.player,
            streak_count: record.streak_count,
            combo_multiplier_bps: record.combo_multiplier_bps,
            expires_at_ledger: record.expires_at_ledger,
            has_snapshot: true,
        }
    }

    pub fn get_expiry_risk_accessor(env: Env, player: Address) -> ExpiryRiskAccessor {
        let Some(cfg) = get_config(&env) else {
            return ExpiryRiskAccessor {
                status: ComboRewardsStatus::Unconfigured,
                player,
                has_snapshot: false,
                at_risk: false,
                ledgers_until_expiry: 0,
            };
        };

        let Some(record) = get_player(&env, &player) else {
            return ExpiryRiskAccessor {
                status: if cfg.paused {
                    ComboRewardsStatus::Paused
                } else {
                    ComboRewardsStatus::Active
                },
                player,
                has_snapshot: false,
                at_risk: false,
                ledgers_until_expiry: 0,
            };
        };

        let current = env.ledger().sequence();
        let ledgers_until_expiry = record.expires_at_ledger.saturating_sub(current);
        let at_risk = ledgers_until_expiry <= 100;

        ExpiryRiskAccessor {
            status: if cfg.paused {
                ComboRewardsStatus::Paused
            } else {
                ComboRewardsStatus::Active
            },
            player: record.player,
            has_snapshot: true,
            at_risk,
            ledgers_until_expiry,
        }
    }
}
