use nft_staking::{
    constants::ERR_REWARD_ALREADY_DISTRIBUTED,
    staking_modules::staking_module_type::StakingModuleType,
};

use crate::setup::{
    constants::{
        NO_ERR_MSG, POOL1_TOKEN_ID, POOL2_TOKEN_ID, REWARD_TOKEN_ID, SECONDARY_REWARD_TOKEN_ID_1,
    },
    types::new_nft_transfer,
    ContractSetup,
};

#[test]
fn simple_successful_primary_distribution() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 1);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.distribute_reward(100_000, NO_ERR_MSG);
    setup.assert_pending_reward(100_000);
}

#[test]
fn double_primary_distribution_fails() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 1);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.distribute_reward(100_000, NO_ERR_MSG);
    setup.distribute_reward(100_000, ERR_REWARD_ALREADY_DISTRIBUTED);
}

#[test]
fn primary_distribution_reward_rate_correct_calculation() {
    let reward = 100_000u64;
    let aggregated_score = 100u64;
    let mut setup = ContractSetup::new(nft_staking::contract_obj);

    let reward_rate = reward / aggregated_score;
    setup.set_aggregated_score(StakingModuleType::All, aggregated_score);
    setup.distribute_reward(reward, NO_ERR_MSG);

    setup.assert_reward_rate(StakingModuleType::All, REWARD_TOKEN_ID, 1, reward_rate);
}

#[test]
fn simple_successful_secondary_distribution() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL2_TOKEN_ID, 1, 1)];
    setup.set_token_score(StakingModuleType::SnakesSfts, POOL2_TOKEN_ID, 1);
    setup.set_stake_pool_type(POOL2_TOKEN_ID, StakingModuleType::SnakesSfts);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.register_token_as_eligible_reward_for_pool(
        SECONDARY_REWARD_TOKEN_ID_1,
        StakingModuleType::SnakesSfts,
    );

    setup.distribute_secondary_reward(
        SECONDARY_REWARD_TOKEN_ID_1,
        POOL2_TOKEN_ID,
        100_000,
        NO_ERR_MSG,
    );
    setup.assert_explicit_pending_reward(SECONDARY_REWARD_TOKEN_ID_1, 100_000);
}

#[test]
fn secondary_distribution_reward_rate_correct_calculation() {
    let reward = 100_000u64;
    let aggregated_score = 100u64;
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL2_TOKEN_ID, StakingModuleType::SnakesSfts);
    setup.set_aggregated_score(StakingModuleType::SnakesSfts, aggregated_score);

    let reward_rate = reward / aggregated_score;

    setup.distribute_secondary_reward(
        SECONDARY_REWARD_TOKEN_ID_1,
        POOL2_TOKEN_ID,
        100_000,
        NO_ERR_MSG,
    );

    setup.assert_reward_rate(
        StakingModuleType::SnakesSfts,
        SECONDARY_REWARD_TOKEN_ID_1,
        1,
        reward_rate,
    );
}

/// This test is to ensure that the reward rate is calculated correctly when there are multiple
/// pools with the same staking module type but also with individual rewards
/// The reward rate should be calculated based on the aggregated score of the staking module type.
/// The primary reward distribution is using the aggregated score of the staking module type "All"
/// meanwhile the secondary reward distribution is using the aggregated score of the specific staking module
#[test]
fn mixed_successful_distribution() {
    let primary_reward_amount = 100_000u64;
    let primary_aggregated_score = 100u64;
    let primary_user_score = 10u64;

    let secondary_reward_amount = 200_000u64;
    let secondary_aggregated_score = 10u64;
    let secondary_user_score = 2u64;

    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.set_token_score(
        StakingModuleType::All,
        POOL1_TOKEN_ID,
        primary_user_score as usize,
    );

    setup.set_stake_pool_type(POOL2_TOKEN_ID, StakingModuleType::SnakesSfts);
    setup.set_token_score(
        StakingModuleType::SnakesSfts,
        POOL2_TOKEN_ID,
        secondary_user_score as usize,
    );
    setup.register_token_as_eligible_reward_for_pool(
        SECONDARY_REWARD_TOKEN_ID_1,
        StakingModuleType::SnakesSfts,
    );

    let primary_transfer = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    let secondary_transfer = vec![new_nft_transfer(POOL2_TOKEN_ID, 1, 1)];
    setup.stake(&primary_transfer, NO_ERR_MSG);
    setup.stake(&secondary_transfer, NO_ERR_MSG);

    setup.set_aggregated_score(StakingModuleType::All, primary_aggregated_score);
    setup.set_aggregated_score(StakingModuleType::SnakesSfts, secondary_aggregated_score);
    setup.distribute_reward(primary_reward_amount, NO_ERR_MSG);
    setup.distribute_secondary_reward(
        SECONDARY_REWARD_TOKEN_ID_1,
        POOL2_TOKEN_ID,
        secondary_reward_amount,
        NO_ERR_MSG,
    );

    let expected_primary_reward =
        primary_reward_amount * primary_user_score / primary_aggregated_score;
    let expected_secondary_reward =
        secondary_reward_amount * secondary_user_score / secondary_aggregated_score;

    setup.assert_aggregated_score(StakingModuleType::All, primary_aggregated_score);
    setup.assert_aggregated_score(StakingModuleType::SnakesSfts, secondary_aggregated_score);

    setup.assert_user_score(StakingModuleType::All, primary_user_score);
    setup.assert_user_score(StakingModuleType::SnakesSfts, secondary_user_score);

    setup.assert_pending_reward(expected_primary_reward);
    setup.assert_explicit_pending_reward(SECONDARY_REWARD_TOKEN_ID_1, expected_secondary_reward);
}
