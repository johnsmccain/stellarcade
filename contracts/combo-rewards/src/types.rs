#![allow(dead_code)]

use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Eq, PartialEq)]
pub enum ComboRewardsStatus {
    Unconfigured = 0,
    Active = 1,
    Paused = 2,
}

#[contracttype]
#[derive(Clone)]
pub struct ComboRewardsConfig {
    pub admin: Address,
    pub paused: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct StreakComboRecord {
    pub player: Address,
    pub streak_count: u32,
    pub combo_multiplier_bps: u32,
    pub expires_at_ledger: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct StreakComboSnapshot {
    pub status: ComboRewardsStatus,
    pub player: Address,
    pub streak_count: u32,
    pub combo_multiplier_bps: u32,
    pub expires_at_ledger: u32,
    pub has_snapshot: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct ExpiryRiskAccessor {
    pub status: ComboRewardsStatus,
    pub player: Address,
    pub has_snapshot: bool,
    pub at_risk: bool,
    pub ledgers_until_expiry: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Config,
    Player(Address),
}
