extern crate std;

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    Address, Env,
};

fn setup(env: &Env) -> (LootRotationClient<'_>, Address) {
    let admin = Address::generate(env);
    let contract_id = env.register_contract(None, LootRotation);
    let client = LootRotationClient::new(env, &contract_id);
    env.mock_all_auths();
    client.init(&admin);
    (client, admin)
}

#[test]
fn active_pool_snapshot_and_rollover_delay_success_path() {
    let env = Env::default();
    env.ledger().with_mut(|ledger| ledger.timestamp = 125);
    let (client, admin) = setup(&env);

    client.set_active_pool(&admin, &7, &12, &250, &100, &200);

    let snapshot = client.active_pool_snapshot();
    assert!(snapshot.configured);
    assert!(snapshot.has_active_pool);
    assert_eq!(snapshot.pool_id, 7);
    assert_eq!(snapshot.item_count, 12);
    assert_eq!(snapshot.seconds_until_rollover, 75);

    let delay = client.rollover_delay();
    assert!(delay.has_active_pool);
    assert!(!delay.rollover_due);
    assert_eq!(delay.seconds_until_rollover, 75);
}

#[test]
fn empty_pool_returns_predictable_zero_state() {
    let env = Env::default();
    let (client, _admin) = setup(&env);

    let snapshot = client.active_pool_snapshot();
    assert!(snapshot.configured);
    assert!(!snapshot.has_active_pool);
    assert_eq!(snapshot.pool_id, 0);
    assert_eq!(snapshot.seconds_until_rollover, 0);

    let delay = client.rollover_delay();
    assert!(!delay.has_active_pool);
    assert!(!delay.rollover_due);
}

#[test]
fn uninitialized_pool_reads_are_predictable() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LootRotation);
    let client = LootRotationClient::new(&env, &contract_id);

    let snapshot = client.active_pool_snapshot();
    assert!(!snapshot.configured);
    assert!(!snapshot.has_active_pool);
    assert_eq!(snapshot.seconds_until_rollover, 0);

    let delay = client.rollover_delay();
    assert!(!delay.configured);
    assert!(!delay.rollover_due);
}

#[test]
fn rollover_due_after_end_time_and_paused_state_is_visible() {
    let env = Env::default();
    env.ledger().with_mut(|ledger| ledger.timestamp = 250);
    let (client, admin) = setup(&env);

    client.set_active_pool(&admin, &3, &5, &100, &10, &200);
    client.set_paused(&admin, &true);

    let delay = client.rollover_delay();
    assert!(delay.paused);
    assert!(delay.rollover_due);
    assert_eq!(delay.seconds_until_rollover, 0);
}
