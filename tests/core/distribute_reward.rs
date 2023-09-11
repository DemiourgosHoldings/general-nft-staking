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
