use multiversx_sc::types::{BigUint, ManagedAddress};

use super::staking_module_type::VestaStakingModule;

pub struct InvalidStakingModule {}

impl InvalidStakingModule {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, C> VestaStakingModule<'a, C> for InvalidStakingModule
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    fn get_base_user_score(&self, _address: &ManagedAddress<C::Api>) -> BigUint<C::Api> {
        BigUint::zero()
    }

    fn get_final_user_score(&self, _address: &ManagedAddress<C::Api>) -> BigUint<C::Api> {
        BigUint::zero()
    }
}
