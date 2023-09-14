use multiversx_sc::types::{BigUint, ManagedAddress, ManagedVec, TokenIdentifier};

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
    sc_ref: &'a C,
    impl_token_id: TokenIdentifier<C::Api>,
    default_impl: DefaultStakingModule<'a, C>,
    user_address: ManagedAddress<C::Api>,
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
        let default_impl = DefaultStakingModule::new(
            sc_ref,
            impl_token_id.clone(),
            user_address.clone(),
            module_type,
        );
        Self {
            sc_ref,
            impl_token_id,
            default_impl,
            user_address,
        }
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

        self.default_impl.get_user_score_temp(staking_module_type)
    }

    fn add_to_storage(&self, nonce: u64, amount: BigUint<C::Api>) {
        let mut staked_nfts = self
            .sc_ref
            .staked_nfts(&self.impl_token_id)
            .remove(&self.user_address)
            .unwrap_or_else(|| ManagedVec::new());

        let existing_item_index = staked_nfts.iter().position(|p| p.nonce == nonce);
        let item_to_insert;
        if existing_item_index.is_none() {
            item_to_insert = NonceQtyPair {
                nonce,
                quantity: amount,
            };
        } else {
            let index_to_remove = existing_item_index.unwrap();
            let existing_item = staked_nfts.get(index_to_remove);
            staked_nfts.remove(index_to_remove);
            item_to_insert = NonceQtyPair {
                nonce,
                quantity: existing_item.quantity + amount,
            };
        }

        staked_nfts.push(item_to_insert);

        self.sc_ref
            .staked_nfts(&self.impl_token_id)
            .insert(self.user_address.clone(), staked_nfts);
    }

    fn start_unbonding(&self, payload: StartUnbondingPayload<<C>::Api>) -> bool {
        self.default_impl.start_unbonding(payload)
    }

    fn get_final_secondary_score(&self) -> BigUint<<C>::Api> {
        self.default_impl
            .get_user_score_temp(&StakingModuleType::SnakesSfts)
    }
}
