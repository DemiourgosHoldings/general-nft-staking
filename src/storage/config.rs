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
}
