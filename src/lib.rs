#![no_std]

multiversx_sc::imports!();

pub mod constants;
pub mod staking_modules;
pub mod storage;
pub mod types;

/// An lib contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait NftStakingContract:
    storage::config::ConfigModule + storage::score::ScoreStorageModule
{
    #[init]
    fn init(&self) {}

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {}

    #[endpoint(startUnbonding)]
    fn start_unbonding(
        &self,
        _token_identifier: TokenIdentifier,
        _nonces: MultiValueManagedVec<u64>,
    ) {
    }

    #[endpoint(claimUnbonded)]
    fn claim_unbonded(&self) {}

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {}
}
