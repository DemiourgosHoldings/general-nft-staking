use core::marker::PhantomData;

use multiversx_sc::types::BigUint;

use super::staking_module_type::{StakingModuleType, VestaStakingModule};

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
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn get_base_user_score(&self, _: &StakingModuleType) -> BigUint<C::Api> {
        BigUint::zero()
    }

    fn get_final_user_score(&self) -> BigUint<C::Api> {
        BigUint::zero()
    }

    fn add_to_storage(&self, _nonce: u64, _amount: BigUint<C::Api>) {}

    fn start_unbonding(
        &self,
        _payload: crate::types::start_unbonding_payload::StartUnbondingPayload<<C>::Api>,
    ) -> bool {
        true
    }

    fn get_final_secondary_score(&self) -> BigUint<<C>::Api> {
        BigUint::zero()
    }
}
