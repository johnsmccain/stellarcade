#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{BonusVaultContract, BonusVaultContractClient};

#[test]
fn accrual_pressure_and_threshold_accessor_happy_path() {
    let env = Env::default();
    let id = env.register(BonusVaultContract, ());
    let client = BonusVaultContractClient::new(&env, &id);
    let admin = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin);
    client.set_state(&admin, &600, &1000);

    let summary = client.get_accrual_pressure_summary();
    assert_eq!(summary.pending_accrual, 600);
    assert_eq!(summary.release_threshold, 1000);
    assert_eq!(summary.pressure_bps, 6000);
    assert!(!summary.over_threshold);

    let accessor = client.get_release_threshold_accessor();
    assert!(accessor.threshold_configured);
    assert_eq!(accessor.remaining_until_release, 400);
}

#[test]
fn accrual_pressure_unconfigured_state() {
    let env = Env::default();
    let id = env.register(BonusVaultContract, ());
    let client = BonusVaultContractClient::new(&env, &id);

    let summary = client.get_accrual_pressure_summary();
    assert_eq!(summary.pending_accrual, 0);
    assert_eq!(summary.release_threshold, 0);
    assert_eq!(summary.pressure_bps, 0);

    let accessor = client.get_release_threshold_accessor();
    assert!(!accessor.threshold_configured);
    assert_eq!(accessor.remaining_until_release, 0);
}
