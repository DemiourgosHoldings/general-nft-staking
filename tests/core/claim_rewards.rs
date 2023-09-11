use nft_staking::{
    constants::ERR_NOTHING_TO_CLAIM, staking_modules::staking_module_type::StakingModuleType,
};

use crate::setup::{
    constants::{NO_ERR_MSG, POOL1_TOKEN_ID, REWARD_TOKEN_ID},
    types::new_nft_transfer,
    ContractSetup,
};

#[test]
fn successful_claim_rewards() {
    let aggregated_score = 100_000;
    let reward = 100_000;
    let single_nft_score = 100;

    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, single_nft_score);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.set_aggregated_score(StakingModuleType::All, aggregated_score);
    setup.distribute_reward(reward, NO_ERR_MSG);

    let expected_reward = reward * single_nft_score as u64 / aggregated_score;

    setup.assert_pending_reward(expected_reward);
    setup.claim_rewards(NO_ERR_MSG);
    setup.assert_pending_reward(0);
    setup.assert_user_token_balance(REWARD_TOKEN_ID, 0, expected_reward);
}

#[test]
fn double_claim_rewards_fail() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 100);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.distribute_reward(100_000, NO_ERR_MSG);
    setup.claim_rewards(NO_ERR_MSG);
    setup.claim_rewards(ERR_NOTHING_TO_CLAIM);
}

#[test]
#[allow(deprecated)]
fn all_epochs_claimed() {
    let distributed_reward = 100_000;
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 100);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.distribute_reward(distributed_reward, NO_ERR_MSG);

    for epoch in 2..=10 {
        setup.b_mock.set_block_epoch(epoch);
        setup.distribute_reward(distributed_reward, NO_ERR_MSG);
    }

    setup.claim_rewards(NO_ERR_MSG);
    setup.assert_pending_reward(0);
    setup.assert_user_token_balance(REWARD_TOKEN_ID, 0, 10 * distributed_reward);
}

#[test]
#[allow(deprecated)]
fn not_distributed_epoch_not_claimed() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 100);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.distribute_reward(100_000, NO_ERR_MSG);
    setup.b_mock.set_block_epoch(2);
    setup.claim_rewards(NO_ERR_MSG);
    setup.distribute_reward(100_000, NO_ERR_MSG);
    setup.assert_pending_reward(100_000);
}
