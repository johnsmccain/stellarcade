#![allow(dead_code)]

use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Eq, PartialEq)]
pub enum RoundFinalizerStatus {
    Unconfigured = 0,
    Active = 1,
    Paused = 2,
}

#[contracttype]
#[derive(Clone)]
pub struct RoundFinalizerConfig {
    pub admin: Address,
    pub paused: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct RoundRecord {
    pub round_id: u64,
    pub unresolved_ops: u32,
    pub has_checkpoint: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct UnresolvedRoundSummary {
    pub status: RoundFinalizerStatus,
    pub total_rounds: u32,
    pub unresolved_rounds: u32,
    pub unresolved_ops: u32,
    pub next_unresolved_round_id: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct FinalizeReadiness {
    pub status: RoundFinalizerStatus,
    pub round_id: u64,
    pub is_ready: bool,
    pub unresolved_ops: u32,
    pub missing_checkpoint: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Config,
    RoundIds,
    Round(u64),
}
