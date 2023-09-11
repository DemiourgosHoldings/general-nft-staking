use nft_staking::staking_modules::staking_module_type::StakingModuleType;

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
        new_nft_transfer(POOL1_TOKEN_ID, 3, 1),
    ];

    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, 50);
    setup.set_token_nonce_score(StakingModuleType::All, POOL1_TOKEN_ID, 3, 100);
    setup.stake(&transfers, NO_ERR_MSG);
    setup.assert_user_score(StakingModuleType::All, 200);
}

#[test]
fn realistic_take_1() {
    let regular_nft_score = 2;
    let legendary_nonce_score = 160;

    let regular_nfts_to_stake = 10;
    let legendary_nfts_to_stake = 10;

    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.set_token_score(StakingModuleType::All, POOL1_TOKEN_ID, regular_nft_score);

    let mut transfers = vec![];

    for nonce in 1..=regular_nfts_to_stake {
        transfers.push(new_nft_transfer(POOL1_TOKEN_ID, nonce, 1));
    }

    for nonce in regular_nfts_to_stake + 1..=(regular_nfts_to_stake + legendary_nfts_to_stake) {
        transfers.push(new_nft_transfer(POOL1_TOKEN_ID, nonce, 1));
        setup.set_token_nonce_score(
            StakingModuleType::All,
            POOL1_TOKEN_ID,
            nonce,
            legendary_nonce_score,
        );
    }

    let expected_score = legendary_nonce_score as u64 * legendary_nfts_to_stake
        + regular_nft_score as u64 * regular_nfts_to_stake;

    setup.stake(&transfers, NO_ERR_MSG);
    setup.assert_user_score(StakingModuleType::All, expected_score);
}
