use crate::staking_modules::staking_module_type::StakingModuleType;
multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ConfigModule {
    #[view(getStakingPoolTypeConfiguration)]
    #[storage_mapper("staking_pool_type_configuration")]
    fn stake_pool_type_configuration(
        &self,
        token_identifier: &TokenIdentifier,
    ) -> SingleValueMapper<StakingModuleType>;

    #[view(getUnbondingTimePenalty)]
    #[storage_mapper("unbonding_time_penalty")]
    fn unbonding_time_penalty(&self) -> SingleValueMapper<u64>;

    #[view(getRewardTokenIdentifier)]
    #[storage_mapper("reward_token_identifier")]
    fn reward_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;
}
