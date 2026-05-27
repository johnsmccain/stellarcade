#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{RoundFinalizerContract, RoundFinalizerContractClient};

#[test]
fn unresolved_summary_and_readiness_happy_path() {
    let env = Env::default();
    let id = env.register(RoundFinalizerContract, ());
    let client = RoundFinalizerContractClient::new(&env, &id);
    let admin = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin);
    client.upsert_round(&admin, &10, &2, &true);
    client.upsert_round(&admin, &11, &0, &true);

    let summary = client.get_unresolved_round_summary();
    assert_eq!(summary.total_rounds, 2);
    assert_eq!(summary.unresolved_rounds, 1);
    assert_eq!(summary.unresolved_ops, 2);
    assert_eq!(summary.next_unresolved_round_id, 10);

    let readiness = client.get_finalize_readiness(&11);
    assert!(readiness.is_ready);
    assert_eq!(readiness.unresolved_ops, 0);
    assert!(!readiness.missing_checkpoint);
}

#[test]
fn unresolved_summary_unconfigured_and_missing_round() {
    let env = Env::default();
    let id = env.register(RoundFinalizerContract, ());
    let client = RoundFinalizerContractClient::new(&env, &id);

    let summary = client.get_unresolved_round_summary();
    assert_eq!(summary.total_rounds, 0);
    assert_eq!(summary.unresolved_rounds, 0);

    let readiness = client.get_finalize_readiness(&99);
    assert!(!readiness.is_ready);
    assert!(readiness.missing_checkpoint);
}
