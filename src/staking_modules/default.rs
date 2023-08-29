use multiversx_sc::types::{BigUint, ManagedAddress, TokenIdentifier};

use super::staking_module_type::VestaStakingModule;

pub struct DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    sc_ref: &'a C,
    impl_token_id: TokenIdentifier<C::Api>,
}

impl<'a, C> DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    pub fn new(sc_ref: &'a C, impl_token_id: TokenIdentifier<C::Api>) -> Self {
        Self {
            sc_ref,
            impl_token_id,
        }
    }
}

impl<'a, C> VestaStakingModule<'a, C> for DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    fn get_base_user_score(&self, _address: &ManagedAddress<C::Api>) -> BigUint<C::Api> {
        todo!()
    }

    fn get_final_user_score(&self, _address: &ManagedAddress<C::Api>) -> BigUint<C::Api> {
        todo!()
    }
}
