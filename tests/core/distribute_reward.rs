use nft_staking::staking_modules::staking_module_type::StakingModuleType;

use crate::setup::{
    constants::{NO_ERR_MSG, POOL1_TOKEN_ID},
    types::new_nft_transfer,
    ContractSetup,
};

#[test]
#[allow(deprecated)]
fn simple_successful_distribution() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![new_nft_transfer(POOL1_TOKEN_ID, 1, 1)];
    setup.set_token_score(POOL1_TOKEN_ID, 1);
    setup.set_stake_pool_type(POOL1_TOKEN_ID, StakingModuleType::XBunnies);
    setup.stake(&transfers, NO_ERR_MSG);

    setup.distribute_reward(100_000, NO_ERR_MSG);
    setup.assert_pending_reward(100_000);
}