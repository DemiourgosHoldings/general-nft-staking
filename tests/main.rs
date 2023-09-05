mod core;
mod module_tests;
mod setup;
use crate::setup::ContractSetup;

#[test]
fn init() {
    let _ = ContractSetup::new(nft_staking::contract_obj);
}
