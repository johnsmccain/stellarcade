#![allow(dead_code)]

use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Eq, PartialEq)]
pub enum RoutingStatus {
    Unconfigured = 0,
    Active = 1,
    Paused = 2,
}

#[contracttype]
#[derive(Clone)]
pub struct RoutingConfig {
    pub admin: Address,
    pub paused: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct RouteRecord {
    pub route_id: u64,
    pub capacity_units: u32,
    pub used_units: u32,
    pub failover_target_configured: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct RouteSaturationSummary {
    pub status: RoutingStatus,
    pub total_routes: u32,
    pub saturated_routes: u32,
    pub average_utilization_bps: u32,
    pub max_utilization_bps: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct FailoverReadiness {
    pub status: RoutingStatus,
    pub route_id: u64,
    pub is_ready: bool,
    pub utilization_bps: u32,
    pub missing_failover_target: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Config,
    RouteIds,
    Route(u64),
}
