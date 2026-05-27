#![allow(dead_code)]

use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Eq, PartialEq)]
pub enum BonusVaultStatus {
    Unconfigured = 0,
    Active = 1,
    Paused = 2,
}

#[contracttype]
#[derive(Clone)]
pub struct BonusVaultConfig {
    pub admin: Address,
    pub paused: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct BonusVaultState {
    pub pending_accrual: i128,
    pub release_threshold: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct AccrualPressureSummary {
    pub status: BonusVaultStatus,
    pub pending_accrual: i128,
    pub release_threshold: i128,
    pub pressure_bps: u32,
    pub over_threshold: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct ReleaseThresholdAccessor {
    pub status: BonusVaultStatus,
    pub threshold_configured: bool,
    pub release_threshold: i128,
    pub remaining_until_release: i128,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Config,
    State,
}
