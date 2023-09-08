use nft_staking::staking_modules::staking_module_type::StakingModuleType;

use crate::setup::{
    constants::{NO_ERR_MSG, POOL2_TOKEN_ID},
    types::new_nft_transfer,
    ContractSetup,
};

// test that this collection won't receive any of the primary rewards
// but will receive 100% of the secondary rewards generated by their subsidiary
#[test]
fn realistic_take_1() {
    let snake_sfts_to_stake = 100;

    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    setup.set_stake_pool_type(POOL2_TOKEN_ID, StakingModuleType::SnakesSfts);
    setup.set_token_score(POOL2_TOKEN_ID, 0);
    setup.set_secondary_token_score(POOL2_TOKEN_ID, 1);

    let transfers = vec![new_nft_transfer(POOL2_TOKEN_ID, 1, snake_sfts_to_stake)];

    setup.stake(&transfers, NO_ERR_MSG);
    setup.assert_user_score(0);
    setup.assert_secondary_user_score(StakingModuleType::SnakesSfts, snake_sfts_to_stake);
}
