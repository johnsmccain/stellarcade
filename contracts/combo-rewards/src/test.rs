#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{ComboRewardsContract, ComboRewardsContractClient};

#[test]
fn streak_combo_snapshot_and_expiry_risk_happy_path() {
    let env = Env::default();
    let id = env.register(ComboRewardsContract, ());
    let client = ComboRewardsContractClient::new(&env, &id);
    let admin = Address::generate(&env);
    let player = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin);
    let now = env.ledger().sequence();
    client.upsert_player_snapshot(&admin, &player, &7, &14500, &(now + 250));

    let snapshot = client.get_streak_combo_snapshot(&player);
    assert!(snapshot.has_snapshot);
    assert_eq!(snapshot.streak_count, 7);
    assert_eq!(snapshot.combo_multiplier_bps, 14500);

    let risk = client.get_expiry_risk_accessor(&player);
    assert!(risk.has_snapshot);
    assert!(!risk.at_risk);
}

#[test]
fn streak_combo_snapshot_missing_state() {
    let env = Env::default();
    let id = env.register(ComboRewardsContract, ());
    let client = ComboRewardsContractClient::new(&env, &id);
    let player = Address::generate(&env);

    let snapshot = client.get_streak_combo_snapshot(&player);
    assert!(!snapshot.has_snapshot);
    assert_eq!(snapshot.streak_count, 0);

    let risk = client.get_expiry_risk_accessor(&player);
    assert!(!risk.has_snapshot);
    assert!(!risk.at_risk);
}
