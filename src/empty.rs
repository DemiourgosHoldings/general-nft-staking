#![no_std]

multiversx_sc::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait NftStakingContract {
    #[init]
    fn init(&self) {}

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self, _pool_id: u64) {}

    #[endpoint(startUnbonding)]
    fn start_unbonding(&self, _pool_id: u64, _nonces: MultiValueManagedVec<u64>) {}

    #[endpoint(claimUnbonded)]
    fn claim_unbonded(&self, _pool_id: u64) {}

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {}
}
