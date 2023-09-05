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
