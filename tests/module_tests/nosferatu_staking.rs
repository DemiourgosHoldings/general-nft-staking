use nft_staking::staking_modules::staking_module_type::StakingModuleType;

use crate::setup::{
    constants::{NO_ERR_MSG, POOL1_TOKEN_ID},
    types::new_nft_transfer,
    ContractSetup,
};

#[test]
fn realistic_take_1() {
    let common_nft_score = 1;
    let rare_nft_score = 3;
    let epic_nft_score = 4;
    let legendary_nonce_score = 11;

    let common_nfts_to_stake = 10;
    let rare_nfts_to_stake = 10;
    let epic_nfts_to_stake = 10;
    let legendary_nfts_to_stake = 10;

    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::Nosferatu);
    setup.set_token_score(POOL1_TOKEN_ID, common_nft_score);

    let mut transfers = vec![];
    let mut nonce_offset = 0;

    for nonce in 1..=common_nfts_to_stake {
        transfers.push(new_nft_transfer(POOL1_TOKEN_ID, nonce, 1));
    }
    nonce_offset += common_nfts_to_stake;

    for nonce in 1..=rare_nfts_to_stake {
        let actual_nft_nonce = nonce + nonce_offset;
        transfers.push(new_nft_transfer(POOL1_TOKEN_ID, actual_nft_nonce, 1));
        setup.set_token_nonce_score(POOL1_TOKEN_ID, actual_nft_nonce, rare_nft_score);
    }
    nonce_offset += rare_nfts_to_stake;

    for nonce in 1..=epic_nfts_to_stake {
        let actual_nft_nonce = nonce + nonce_offset;
        transfers.push(new_nft_transfer(POOL1_TOKEN_ID, actual_nft_nonce, 1));
        setup.set_token_nonce_score(POOL1_TOKEN_ID, actual_nft_nonce, epic_nft_score);
    }
    nonce_offset += epic_nfts_to_stake;

    for nonce in 1..=legendary_nfts_to_stake {
        let actual_nft_nonce = nonce + nonce_offset;
        transfers.push(new_nft_transfer(POOL1_TOKEN_ID, actual_nft_nonce, 1));
        setup.set_token_nonce_score(POOL1_TOKEN_ID, actual_nft_nonce, legendary_nonce_score);
    }

    let expected_score = legendary_nonce_score as u64 * legendary_nfts_to_stake
        + epic_nft_score as u64 * epic_nfts_to_stake
        + rare_nft_score as u64 * rare_nfts_to_stake
        + common_nft_score as u64 * common_nfts_to_stake;

    setup.stake(&transfers, NO_ERR_MSG);
    setup.assert_user_score(expected_score);
}
