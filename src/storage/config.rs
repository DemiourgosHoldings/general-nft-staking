use crate::staking_modules::staking_module_type::StakingModuleType;
multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ConfigModule {
    #[view(getStakingPoolTypeConfiguration)]
    #[storage_mapper("staking_pool_type_configuration")]
    fn stake_pool_type_configuration(
        &self,
        collection_token_identifier: &TokenIdentifier,
    ) -> SingleValueMapper<StakingModuleType>;

    #[view(getStakingModulesByRewardToken)]
    #[storage_mapper("reward_token_to_staking_module_map")]
    fn reward_token_to_staking_module_map(
        &self,
        reward_token_id: &TokenIdentifier,
    ) -> UnorderedSetMapper<StakingModuleType>;

    #[view(getUnbondingTimePenalty)]
    #[storage_mapper("unbonding_time_penalty")]
    fn unbonding_time_penalty(&self) -> SingleValueMapper<u64>;

    #[view(getRewardTokenIdentifiers)]
    #[storage_mapper("reward_token_identifiers")]
    fn reward_token_identifiers(&self) -> UnorderedSetMapper<TokenIdentifier>;

    #[view(getPrimaryRewardTokenIdentifier)]
    #[storage_mapper("primary_reward_token_identifier")]
    fn primary_reward_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getEligibleStakeTokenIdentifiers)]
    #[storage_mapper("eligible_stake_token_identifiers")]
    fn eligible_stake_token_identifiers(&self) -> UnorderedSetMapper<TokenIdentifier>;
}
