use multiversx_sc::types::{BigUint, ManagedAddress, TokenIdentifier};

use super::{
    default::DefaultStakingModule,
    staking_module_type::{StakingModuleType, VestaStakingModule},
};
use crate::types::{nonce_qty_pair::NonceQtyPair, start_unbonding_payload::StartUnbondingPayload};

pub struct SnakesSftStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    default_impl: DefaultStakingModule<'a, C>,
}

impl<'a, C> SnakesSftStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    pub fn new(
        sc_ref: &'a C,
        impl_token_id: TokenIdentifier<C::Api>,
        user_address: ManagedAddress<C::Api>,
        module_type: StakingModuleType,
    ) -> Self {
        let default_impl =
            DefaultStakingModule::new(sc_ref, impl_token_id.clone(), user_address, module_type);
        Self { default_impl }
    }
}

impl<'a, C> VestaStakingModule<'a, C> for SnakesSftStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    fn get_base_user_score(&self, staking_module_type: &StakingModuleType) -> BigUint<C::Api> {
        // Snakes don't get any share from the primary reward consisting of 15% of total AURYN emission
        // because they get a share from the secondary reward consisting of 5% of total AURYN emission
        // Hence why the base score for "All" is 0
        if staking_module_type == &StakingModuleType::All {
            return BigUint::zero();
        }

        self.default_impl.get_base_user_score(staking_module_type)
    }

    fn add_to_storage(&mut self, nonce: u64, amount: BigUint<C::Api>) {
        let existing_item_index = self
            .default_impl
            .staked_assets
            .iter()
            .position(|p| p.nonce == nonce);
        let item_to_insert;
        if existing_item_index.is_none() {
            item_to_insert = NonceQtyPair {
                nonce,
                quantity: amount,
            };
        } else {
            let index_to_remove = existing_item_index.unwrap();
            let existing_item = self.default_impl.staked_assets.get(index_to_remove);
            self.default_impl.staked_assets.remove(index_to_remove);
            item_to_insert = NonceQtyPair {
                nonce,
                quantity: existing_item.quantity + amount,
            };
        }

        self.default_impl.staked_assets.push(item_to_insert);
    }

    fn start_unbonding(&mut self, payload: StartUnbondingPayload<<C>::Api>) -> bool {
        self.default_impl.start_unbonding(payload)
    }
}
