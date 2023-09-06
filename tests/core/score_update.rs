use nft_staking::{
    constants::DEB_DENOMINATION, staking_modules::staking_module_type::StakingModuleType,
};

use crate::setup::{
    constants::{NO_ERR_MSG, POOL1_TOKEN_ID},
    types::new_nft_transfer,
    ContractSetup,
};

#[test]
fn update_deb_triggers_secure_rewards() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_token_score(POOL1_TOKEN_ID, 1);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);
    setup.set_aggregated_score(10);
    setup.distribute_reward(100_000, NO_ERR_MSG);
    setup.assert_pending_reward(10_000);

    setup.update_user_deb(2 * DEB_DENOMINATION);
    setup.assert_stored_rewards(10_000);
}

#[test]
fn update_deb_updates_user_score() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_token_score(POOL1_TOKEN_ID, 1);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.update_user_deb(2 * DEB_DENOMINATION);
    setup.assert_user_score(2);
}

#[test]
fn update_deb_updates_general_score() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_token_score(POOL1_TOKEN_ID, 1);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.update_user_deb(2 * DEB_DENOMINATION);
    setup.assert_aggregated_score(2);
}

#[test]
fn update_deb_with_score_zero_success() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);

    setup.update_user_deb(2 * DEB_DENOMINATION);
    setup.assert_aggregated_score(0);
}
