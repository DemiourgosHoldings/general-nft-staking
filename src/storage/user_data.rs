use crate::types::nonce_qty_pair::NonceQtyPair;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait UserDataStorageModule {
    #[view(getStakedNfts)]
    #[storage_mapper("staked_nfts")]
    fn staked_nfts(
        &self,
        token_identifier: &TokenIdentifier,
    ) -> MapMapper<ManagedAddress, ManagedVec<NonceQtyPair<Self::Api>>>;

    #[view(getUserDeb)]
    #[storage_mapper("user_deb")]
    fn user_deb(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getAggregatedStakingScore)]
    #[storage_mapper("aggregated_staking_score")]
    fn aggregated_staking_score(&self) -> SingleValueMapper<BigUint>;

    #[view(getAggregatedUserStakingScore)]
    #[storage_mapper("aggregated_user_staking_score")]
    fn aggregated_user_staking_score(&self, address: &ManagedAddress)
        -> SingleValueMapper<BigUint>;

    #[view(getPendingRewards)]
    #[storage_mapper("pending_rewards")]
    fn pending_rewards(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
