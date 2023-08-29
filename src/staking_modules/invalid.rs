use core::marker::PhantomData;

use multiversx_sc::types::BigUint;

use super::staking_module_type::VestaStakingModule;

pub struct InvalidStakingModule<'a, C> {
    _phantom: PhantomData<&'a C>,
}

impl<'a, C> InvalidStakingModule<'a, C> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<'a, C> VestaStakingModule<'a, C> for InvalidStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    fn get_base_user_score(&self) -> BigUint<C::Api> {
        BigUint::zero()
    }

    fn get_final_user_score(&self) -> BigUint<C::Api> {
        BigUint::zero()
    }

    fn add_to_storage(&self, _nonce: u64, _amount: BigUint<C::Api>) {}
}
