use crate::{
    staking_modules::staking_module_type::StakingModuleType,
    types::{nonce_qty_pair::NonceQtyPair, start_unbonding_payload::StartUnbondingPayload},
};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait UserDataStorageModule {
    // Core storage
    #[view(getStakedNfts)]
    #[storage_mapper("staked_nfts")]
    fn staked_nfts(
        &self,
        token_identifier: &TokenIdentifier,
    ) -> MapMapper<ManagedAddress, ManagedVec<NonceQtyPair<Self::Api>>>;

    #[view(getUnbondingAssets)]
    #[storage_mapper("unbonding_assets")]
    fn unbonding_assets(
        &self,
        address: &ManagedAddress,
    ) -> MapMapper<u64, StartUnbondingPayload<Self::Api>>;

    #[view(getUserDeb)]
    #[storage_mapper("user_deb")]
    fn user_deb(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    // primary reward storage
    #[view(getAggregatedStakingScore)]
    #[storage_mapper("aggregated_staking_score")]
    fn aggregated_staking_score(
        &self,
        staking_module: &StakingModuleType,
    ) -> SingleValueMapper<BigUint>;

    #[view(getAggregatedUserStakingScore)]
    #[storage_mapper("aggregated_user_staking_score")]
    fn aggregated_user_staking_score(
        &self,
        staking_module: &StakingModuleType,
        address: &ManagedAddress,
    ) -> SingleValueMapper<BigUint>;

    #[view(getPendingRewards)]
    #[storage_mapper("pending_rewards")]
    fn pending_rewards(
        &self,
        address: &ManagedAddress,
        token_identifier: &TokenIdentifier,
    ) -> SingleValueMapper<BigUint>;

    #[view(getLastClaimedEpoch)]
    #[storage_mapper("last_claimed_epoch")]
    fn last_claimed_epoch(
        &self,
        staking_module: &StakingModuleType,
        address: &ManagedAddress,
    ) -> SingleValueMapper<u64>;

    #[view(getRewardRate)]
    #[storage_mapper("reward_rate")]
    fn reward_rate(
        &self,
        epoch: u64,
        staking_module: &StakingModuleType,
        reward_token_identifier: &TokenIdentifier,
    ) -> SingleValueMapper<BigUint>;

    #[view(getRewardDistributionTimestamp)]
    #[storage_mapper("reward_distribution_timestamp")]
    fn reward_distribution_timestamp(
        &self,
        epoch: u64,
        token_identifier: &TokenIdentifier,
    ) -> SingleValueMapper<u64>;
}
