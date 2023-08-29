mod setup;
use setup::{
    constants::{NO_ERR_MSG, POOL1_TOKEN_ID},
    types::new_nft_transfer,
};

use crate::setup::ContractSetup;

#[test]
fn init() {
    let _ = ContractSetup::new(nft_staking::contract_obj);
}

#[test]
fn stake() {
    let mut setup = ContractSetup::new(nft_staking::contract_obj);
    let transfers = vec![
        new_nft_transfer(POOL1_TOKEN_ID, 1, 1),
        new_nft_transfer(POOL1_TOKEN_ID, 2, 1),
    ];

    setup.stake(&transfers, NO_ERR_MSG);
}
