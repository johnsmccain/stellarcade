use soroban_sdk::{contracttype, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllocationRoute {
    pub route_id: Symbol,
    pub target_bps: u32,
    pub allocated_amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteDrift {
    pub route_id: Symbol,
    pub target_bps: u32,
    pub allocated_amount: i128,
    pub expected_amount: i128,
    pub drift_amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllocationDriftSummary {
    pub configured: bool,
    pub paused: bool,
    pub route_count: u32,
    pub total_allocated: i128,
    pub target_bps_total: u32,
    pub total_drift: i128,
    pub max_route_drift: i128,
    pub drift_threshold: i128,
    pub target_bps_valid: bool,
    pub balanced: bool,
    pub routes: Vec<RouteDrift>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RebalanceReadiness {
    pub configured: bool,
    pub paused: bool,
    pub route_count: u32,
    pub total_allocated: i128,
    pub target_bps_total: u32,
    pub target_bps_valid: bool,
    pub drift_threshold: i128,
    pub total_drift: i128,
    pub max_route_drift: i128,
    pub has_drift: bool,
    pub ready_to_rebalance: bool,
}
