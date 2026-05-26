use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LootPool {
    pub pool_id: u64,
    pub item_count: u32,
    pub reward_weight: u32,
    pub starts_at: u64,
    pub ends_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivePoolSnapshot {
    pub configured: bool,
    pub paused: bool,
    pub has_active_pool: bool,
    pub pool_id: u64,
    pub item_count: u32,
    pub reward_weight: u32,
    pub starts_at: u64,
    pub ends_at: u64,
    pub now: u64,
    pub seconds_until_rollover: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RolloverDelay {
    pub configured: bool,
    pub paused: bool,
    pub has_active_pool: bool,
    pub rollover_due: bool,
    pub now: u64,
    pub ends_at: u64,
    pub seconds_until_rollover: u64,
}
