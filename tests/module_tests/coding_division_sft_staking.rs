use nft_staking::{
    constants::VESTA_CODING_DIVISION_FULL_SET_MAX_NONCE,
    staking_modules::staking_module_type::StakingModuleType,
};

use crate::setup::{
    constants::{NO_ERR_MSG, POOL1_TOKEN_ID},
    types::new_nft_transfer,
    ContractSetup,
};

#[test]
fn stake() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
    ];

    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::CodingDivisionSfts);
    setup.set_token_score(POOL1_TOKEN_ID, 50);
    setup.stake(&transfers, NO_ERR_MSG);
    setup.assert_user_score(100);
}

#[test]
fn stake_full_set() {
    let one_item_score = 50;
    let full_set_bonus_score = 10;

    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let mut transfers = vec![];
    for nonce in 1..=VESTA_CODING_DIVISION_FULL_SET_MAX_NONCE {
        transfers.push(new_nft_transfer(POOL1_TOKEN_ID, nonce, 1));
    }

    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::CodingDivisionSfts);
    setup.set_token_score(POOL1_TOKEN_ID, one_item_score);
    setup.set_full_set_score(POOL1_TOKEN_ID, full_set_bonus_score);

    setup.stake(&transfers, NO_ERR_MSG);

    setup.assert_user_score(
        VESTA_CODING_DIVISION_FULL_SET_MAX_NONCE * one_item_score as u64
            + full_set_bonus_score as u64,
    );
}
