use crate::{
    staking_modules::staking_module_type::StakingModuleType, types::nonce_qty_pair::NonceQtyPair,
};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ConfigModule {
    #[view(getStakingPoolTypeConfiguration)]
    #[storage_mapper("staking_pool_type_configuration")]
    fn stake_pool_type_configuration(
        &self,
        token_identifier: &TokenIdentifier,
    ) -> SingleValueMapper<StakingModuleType>;

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
}
