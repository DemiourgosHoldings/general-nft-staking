use nft_staking::{
    constants::ERR_NOTHING_TO_CLAIM, staking_modules::staking_module_type::StakingModuleType,
};

use crate::setup::{
    constants::{NO_ERR_MSG, POOL1_TOKEN_ID, REWARD_TOKEN_ID, SECONDARY_REWARD_TOKEN_ID_1},
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

#[test]
fn mixed_rewards_claim_successful() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfer = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];

    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 1);
    setup.set_token_score(StakingModuleType::XBunnies, POOL1_TOKEN_ID, 1);
    setup.register_token_as_eligible_reward_for_pool(
        SECONDARY_REWARD_TOKEN_ID_1,
        StakingModuleType::XBunnies,
    );

    setup.stake(&transfer, NO_ERR_MSG);

    setup.distribute_reward(100_000, NO_ERR_MSG);
    setup.distribute_secondary_reward(
        SECONDARY_REWARD_TOKEN_ID_1,
        POOL1_TOKEN_ID,
        100_000,
        NO_ERR_MSG,
    );

    setup.claim_rewards(NO_ERR_MSG);

    setup.assert_user_token_balance(REWARD_TOKEN_ID, 0, 100_000);
    setup.assert_user_token_balance(SECONDARY_REWARD_TOKEN_ID_1, 0, 100_000);
}

#[test]
fn mixed_partial_rewards_claim_successful() {
    let reward_amount = 100_000;
    let aggregated_score = 10;
    let user_score_general_reward = 1;
    let user_score_secondary_reward = 2;

    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfer = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];

    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.set_token_score(
        StakingModuleType::All,
        POOL1_TOKEN_ID,
        user_score_general_reward,
    );
    setup.set_token_score(
        StakingModuleType::XBunnies,
        POOL1_TOKEN_ID,
        user_score_secondary_reward,
    );
    setup.register_token_as_eligible_reward_for_pool(
        SECONDARY_REWARD_TOKEN_ID_1,
        StakingModuleType::XBunnies,
    );

    setup.stake(&transfer, NO_ERR_MSG);

    setup.set_aggregated_score(StakingModuleType::All, aggregated_score);
    setup.set_aggregated_score(StakingModuleType::XBunnies, aggregated_score);
    setup.distribute_reward(reward_amount, NO_ERR_MSG);
    setup.distribute_secondary_reward(
        SECONDARY_REWARD_TOKEN_ID_1,
        POOL1_TOKEN_ID,
        reward_amount,
        NO_ERR_MSG,
    );

    setup.claim_rewards(NO_ERR_MSG);

    let expected_primary_token_amount =
        reward_amount * user_score_general_reward as u64 / aggregated_score;
    let expected_secondary_token_amount =
        reward_amount * user_score_secondary_reward as u64 / aggregated_score;

    setup.assert_user_token_balance(REWARD_TOKEN_ID, 0, expected_primary_token_amount);
    setup.assert_user_token_balance(
        SECONDARY_REWARD_TOKEN_ID_1,
        0,
        expected_secondary_token_amount,
    );
}
