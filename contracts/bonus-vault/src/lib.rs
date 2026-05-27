#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

mod storage;
mod test;
mod types;

use storage::*;
use types::*;

#[contract]
pub struct BonusVaultContract;

#[contractimpl]
impl BonusVaultContract {
    pub fn initialize(env: Env, admin: Address) {
        if get_config(&env).is_some() {
            panic!("already initialized");
        }
        set_config(
            &env,
            &BonusVaultConfig {
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

    pub fn set_state(
        env: Env,
        admin: Address,
        pending_accrual: i128,
        release_threshold: i128,
    ) {
        let cfg = get_config(&env).unwrap_or_else(|| panic!("not configured"));
        admin.require_auth();
        if admin != cfg.admin {
            panic!("not admin");
        }
        if pending_accrual < 0 || release_threshold < 0 {
            panic!("invalid amount");
        }
        set_state(
            &env,
            &BonusVaultState {
                pending_accrual,
                release_threshold,
            },
        );
    }

    pub fn get_accrual_pressure_summary(env: Env) -> AccrualPressureSummary {
        let Some(cfg) = get_config(&env) else {
            return AccrualPressureSummary {
                status: BonusVaultStatus::Unconfigured,
                pending_accrual: 0,
                release_threshold: 0,
                pressure_bps: 0,
                over_threshold: false,
            };
        };

        let state = get_state(&env).unwrap_or(BonusVaultState {
            pending_accrual: 0,
            release_threshold: 0,
        });

        let pressure_bps = if state.release_threshold <= 0 {
            0
        } else {
            ((state.pending_accrual as u128 * 10_000u128) / state.release_threshold as u128) as u32
        };

        AccrualPressureSummary {
            status: if cfg.paused {
                BonusVaultStatus::Paused
            } else {
                BonusVaultStatus::Active
            },
            pending_accrual: state.pending_accrual,
            release_threshold: state.release_threshold,
            pressure_bps,
            over_threshold: state.release_threshold > 0 && state.pending_accrual >= state.release_threshold,
        }
    }

    pub fn get_release_threshold_accessor(env: Env) -> ReleaseThresholdAccessor {
        let Some(cfg) = get_config(&env) else {
            return ReleaseThresholdAccessor {
                status: BonusVaultStatus::Unconfigured,
                threshold_configured: false,
                release_threshold: 0,
                remaining_until_release: 0,
            };
        };

        let state = get_state(&env).unwrap_or(BonusVaultState {
            pending_accrual: 0,
            release_threshold: 0,
        });

        let threshold_configured = state.release_threshold > 0;
        let remaining_until_release = if !threshold_configured || state.pending_accrual >= state.release_threshold
        {
            0
        } else {
            state.release_threshold - state.pending_accrual
        };

        ReleaseThresholdAccessor {
            status: if cfg.paused {
                BonusVaultStatus::Paused
            } else {
                BonusVaultStatus::Active
            },
            threshold_configured,
            release_threshold: state.release_threshold,
            remaining_until_release,
        }
    }
}
