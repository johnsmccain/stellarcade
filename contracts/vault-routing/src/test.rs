#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{VaultRoutingContract, VaultRoutingContractClient};

#[test]
fn route_saturation_summary_and_failover_readiness_happy_path() {
    let env = Env::default();
    let id = env.register(VaultRoutingContract, ());
    let client = VaultRoutingContractClient::new(&env, &id);
    let admin = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin);
    client.upsert_route(&admin, &1, &100, &95, &true);
    client.upsert_route(&admin, &2, &200, &40, &true);

    let summary = client.get_route_saturation_summary();
    assert_eq!(summary.total_routes, 2);
    assert_eq!(summary.saturated_routes, 1);
    assert!(summary.max_utilization_bps >= 9_500);

    let readiness = client.get_failover_readiness(&2);
    assert!(readiness.is_ready);
    assert!(!readiness.missing_failover_target);
}

#[test]
fn route_saturation_unconfigured_and_missing_route() {
    let env = Env::default();
    let id = env.register(VaultRoutingContract, ());
    let client = VaultRoutingContractClient::new(&env, &id);

    let summary = client.get_route_saturation_summary();
    assert_eq!(summary.total_routes, 0);
    assert_eq!(summary.average_utilization_bps, 0);

    let readiness = client.get_failover_readiness(&100);
    assert!(!readiness.is_ready);
    assert!(readiness.missing_failover_target);
}
