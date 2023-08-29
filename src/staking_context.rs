use crate::staking_modules::staking_module_type::{
    StakingModuleTypeFactory, StakingModuleTypeMapping, VestaStakingModule,
};

multiversx_sc::imports!();

pub struct StakingContext<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    sc_ref: &'a C,
    aggregated_score: BigUint<C::Api>,
    aggregated_user_score: BigUint<C::Api>,
    caller: ManagedAddress<C::Api>,
    staking_module_impl: StakingModuleTypeMapping<'a, C>,
}

impl<'a, C> StakingContext<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    pub fn new(sc_ref: &'a C, payment_token_id: &TokenIdentifier<C::Api>) -> Self {
        let caller = sc_ref.blockchain().get_caller();
        let aggregated_score = sc_ref.aggregated_staking_score().get();
        let aggregated_user_score = match sc_ref.aggregated_user_staking_score(&caller).is_empty() {
            true => BigUint::zero(),
            false => sc_ref.aggregated_user_staking_score(&caller).get(),
        };
        let staking_module_type = sc_ref.stake_pool_type_configuration(payment_token_id).get();
        let staking_module_impl =
            staking_module_type.get_module(sc_ref, payment_token_id.clone(), caller.clone());
        Self {
            sc_ref,
            aggregated_score,
            aggregated_user_score,
            caller,
            staking_module_impl,
        }
    }

    pub fn add_to_stake(&self, payments: &ManagedVec<C::Api, EsdtTokenPayment<C::Api>>) {
        self.secure_rewards();

        for payment in payments.iter() {
            self.staking_module_impl
                .add_to_storage(payment.token_nonce, payment.amount);
        }

        let new_user_score = self.staking_module_impl.get_final_user_score();
        let diff = &new_user_score - &self.aggregated_user_score;

        self.sc_ref
            .aggregated_user_staking_score(&self.caller)
            .set(new_user_score);
        self.sc_ref
            .aggregated_staking_score()
            .set(&self.aggregated_score - &diff);
    }

    fn secure_rewards(&self) {
        let rewards = self.compute_rewards();
        let stored_rewards = match self.sc_ref.pending_rewards(&self.caller).is_empty() {
            true => BigUint::zero(),
            false => self.sc_ref.pending_rewards(&self.caller).get(),
        };

        //TODO: reset staking timestamp
        self.sc_ref
            .pending_rewards(&self.caller)
            .set(rewards + stored_rewards);
    }

    fn compute_rewards(&self) -> BigUint<C::Api> {
        BigUint::zero()
    }
}
