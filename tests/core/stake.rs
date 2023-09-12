use nft_staking::{
    constants::ERR_ONE_TOKEN_ID_SUPPORTED, staking_modules::staking_module_type::StakingModuleType,
};

use crate::setup::{
    constants::{NO_ERR_MSG, POOL1_TOKEN_ID, POOL2_TOKEN_ID},
    types::new_nft_transfer,
    ContractSetup,
};

#[test]
fn single_successful_stake() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);

    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.stake(&transfers, NO_ERR_MSG);
}

#[test]
fn multiple_transfer_successful_stake() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);

    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 3, 1),
    ];
    setup.stake(&transfers, NO_ERR_MSG);
}

#[test]
fn different_token_id_failed_stake() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);

    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
        new_nft_transfer(POOL2_TOKEN_ID, 3, 1),
    ];
    setup.stake(&transfers, ERR_ONE_TOKEN_ID_SUPPORTED);
}

#[test]
fn regular_stake_updates_user_score() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 1);

    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.stake(&transfers, NO_ERR_MSG);
    setup.assert_user_score(StakingModuleType::All, 1);
}

#[test]
fn regular_stake_updates_general_score() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 1);

    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.stake(&transfers, NO_ERR_MSG);
    setup.assert_aggregated_score(StakingModuleType::All, 1);
}

#[test]
fn secondary_pool_stake_updates_user_score() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL2_TOKEN_ID, StakingModuleType::SnakesSfts);
    setup.set_token_score(StakingModuleType::SnakesSfts, POOL2_TOKEN_ID, 1);

    let transfers = vec![new_nft_transfer(POOL2_TOKEN_ID, 1, 1)];
    setup.stake(&transfers, NO_ERR_MSG);
    setup.assert_user_score(StakingModuleType::SnakesSfts, 1);
}

#[test]
fn secondary_pool_stake_updates_general_score() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL2_TOKEN_ID, StakingModuleType::SnakesSfts);
    setup.set_token_score(StakingModuleType::SnakesSfts, POOL2_TOKEN_ID, 1);

    let transfers = vec![new_nft_transfer(POOL2_TOKEN_ID, 1, 1)];
    setup.stake(&transfers, NO_ERR_MSG);
    setup.assert_aggregated_score(StakingModuleType::SnakesSfts, 1);
}

#[test]
fn one_collection_with_multiple_rewards_updates_all_scores() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 1);
    setup.set_token_score(StakingModuleType::XBunnies, POOL1_TOKEN_ID, 1);

    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.stake(&transfers, NO_ERR_MSG);
    setup.assert_user_score(StakingModuleType::All, 1);
    setup.assert_user_score(StakingModuleType::XBunnies, 1);
}

#[test]
fn multiple_collections_only_with_primary_reward_updates_all_scores() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.set_stake_pool_type(POOL2_TOKEN_ID, StakingModuleType::Bloodshed);
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 1);
    setup.set_token_score(StakingModuleType::All, POOL2_TOKEN_ID, 1);

    let transfers_1 = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    let transfers_2 = vec![new_nft_transfer(POOL2_TOKEN_ID, 1, 1)];
    setup.stake(&transfers_1, NO_ERR_MSG);
    setup.stake(&transfers_2, NO_ERR_MSG);

    setup.assert_user_score(StakingModuleType::All, 2);
}

#[test]
fn multiple_collections_with_multiple_rewards_updates_all_scores() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.set_stake_pool_type(POOL2_TOKEN_ID, StakingModuleType::Bloodshed);
    setup.set_token_score(StakingModuleType::XBunnies, POOL1_TOKEN_ID, 1);
    setup.set_token_score(StakingModuleType::Bloodshed, POOL2_TOKEN_ID, 2);
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 100);
    setup.set_token_score(StakingModuleType::All, POOL2_TOKEN_ID, 200);

    let transfers_1 = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    let transfers_2 = vec![new_nft_transfer(POOL2_TOKEN_ID, 1, 1)];

    setup.stake(&transfers_1, NO_ERR_MSG);
    setup.stake(&transfers_2, NO_ERR_MSG);

    setup.assert_user_score(StakingModuleType::XBunnies, 1);
    setup.assert_user_score(StakingModuleType::Bloodshed, 2);
    setup.assert_user_score(StakingModuleType::All, 300);
}

#[test]
fn stake_top_up_updates_score_correctly() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 1);

    let transfers_1 = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.stake(&transfers_1, NO_ERR_MSG);
    setup.assert_user_score(StakingModuleType::All, 1);

    let transfers_2 = vec![new_nft_transfer(POOL1_TOKEN_ID, 2, 1)];
    setup.stake(&transfers_2, NO_ERR_MSG);
    setup.assert_user_score(StakingModuleType::All, 2);
}

#[test]
fn stake_top_up_with_different_score_nfts_updates_score_correctly() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 1);
    setup.set_token_nonce_score(StakingModuleType::All, POOL1_TOKEN_ID, 2, 100);

    let transfers_1 = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.stake(&transfers_1, NO_ERR_MSG);
    setup.assert_user_score(StakingModuleType::All, 1);

    let transfers_2 = vec![new_nft_transfer(POOL1_TOKEN_ID, 2, 1)];
    setup.stake(&transfers_2, NO_ERR_MSG);
    setup.assert_user_score(StakingModuleType::All, 101);
}
