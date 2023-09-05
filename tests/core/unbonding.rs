use nft_staking::{
    constants::ERR_FAILED_UNBONDING, staking_modules::staking_module_type::StakingModuleType,
};

use crate::setup::{
    constants::{NO_ERR_MSG, POOL1_TOKEN_ID, POOL2_TOKEN_ID},
    types::new_nft_transfer,
    ContractSetup,
};

#[test]
fn single_successful_unbonding() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.start_unbonding(POOL1_TOKEN_ID, &[(1, 1)], NO_ERR_MSG);
}

#[test]
fn partial_stake_successful_unbonding() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 3, 1),
    ];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.start_unbonding(POOL1_TOKEN_ID, &[(1, 1), (2, 1)], NO_ERR_MSG);
}

#[test]
fn full_stake_successful_unbonding() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 3, 1),
    ];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.start_unbonding(POOL1_TOKEN_ID, &[(1, 1), (2, 1), (3, 1)], NO_ERR_MSG);
}

#[test]
fn wrong_token_identifier_failed_unbonding() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 3, 1),
    ];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.start_unbonding(POOL2_TOKEN_ID, &[(1, 1)], ERR_FAILED_UNBONDING);
}

#[test]
fn correct_token_identifier_invalid_nonce_failed_unbonding() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 3, 1),
    ];
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.start_unbonding(POOL1_TOKEN_ID, &[(4, 1)], ERR_FAILED_UNBONDING);
}
