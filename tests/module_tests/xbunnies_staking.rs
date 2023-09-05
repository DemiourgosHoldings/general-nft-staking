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
    setup.set_token_score(POOL1_TOKEN_ID, 50);
    setup.set_token_nonce_score(POOL1_TOKEN_ID, 3, 100);
    setup.stake(&transfers, NO_ERR_MSG);
    setup.assert_user_score(200);
}
