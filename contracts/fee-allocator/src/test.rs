extern crate std;

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

fn setup(env: &Env) -> (FeeAllocatorClient<'_>, Address) {
    let admin = Address::generate(env);
    let contract_id = env.register_contract(None, FeeAllocator);
    let client = FeeAllocatorClient::new(env, &contract_id);
    env.mock_all_auths();
    client.init(&admin);
    (client, admin)
}

#[test]
fn drift_summary_and_rebalance_readiness_success_path() {
    let env = Env::default();
    let (client, admin) = setup(&env);

    client.upsert_route(&admin, &symbol_short!("treas"), &5_000, &70);
    client.upsert_route(&admin, &symbol_short!("rewar"), &5_000, &30);

    let summary = client.allocation_drift_summary();
    assert!(summary.configured);
    assert_eq!(summary.route_count, 2);
    assert_eq!(summary.total_allocated, 100);
    assert_eq!(summary.target_bps_total, 10_000);
    assert_eq!(summary.total_drift, 40);
    assert_eq!(summary.max_route_drift, 20);
    assert!(!summary.balanced);

    let readiness = client.rebalance_readiness();
    assert!(readiness.has_drift);
    assert!(readiness.ready_to_rebalance);
}

#[test]
fn empty_routes_return_zero_state() {
    let env = Env::default();
    let (client, _admin) = setup(&env);

    let summary = client.allocation_drift_summary();
    assert!(summary.configured);
    assert_eq!(summary.route_count, 0);
    assert_eq!(summary.total_allocated, 0);
    assert_eq!(summary.total_drift, 0);
    assert!(!summary.target_bps_valid);

    let readiness = client.rebalance_readiness();
    assert!(!readiness.has_drift);
    assert!(!readiness.ready_to_rebalance);
}

#[test]
fn uninitialized_allocator_reads_are_predictable() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FeeAllocator);
    let client = FeeAllocatorClient::new(&env, &contract_id);

    let summary = client.allocation_drift_summary();
    assert!(!summary.configured);
    assert_eq!(summary.route_count, 0);
    assert_eq!(summary.total_drift, 0);

    let readiness = client.rebalance_readiness();
    assert!(!readiness.configured);
    assert!(!readiness.ready_to_rebalance);
}

#[test]
fn paused_allocator_is_not_rebalance_ready() {
    let env = Env::default();
    let (client, admin) = setup(&env);

    client.upsert_route(&admin, &symbol_short!("treas"), &5_000, &80);
    client.upsert_route(&admin, &symbol_short!("rewar"), &5_000, &20);
    client.set_paused(&admin, &true);

    let readiness = client.rebalance_readiness();
    assert!(readiness.paused);
    assert!(readiness.has_drift);
    assert!(!readiness.ready_to_rebalance);
}
