use crate::{
    staking_modules::staking_module_type::{
        StakingModuleType, StakingModuleTypeFactory, StakingModuleTypeMapping, VestaStakingModule,
    },
    types::start_unbonding_payload::StartUnbondingPayload,
    utils::secure_rewards,
};

multiversx_sc::imports!();

pub struct StakingContext<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    sc_ref: &'a C,
    caller: ManagedAddress<C::Api>,
    aggregated_score: BigUint<C::Api>,
    aggregated_user_score: BigUint<C::Api>,
    secondary_aggregated_score: BigUint<C::Api>,
    secondary_aggregated_user_score: BigUint<C::Api>,
    staking_module_type: StakingModuleType,
    staking_module_impl: StakingModuleTypeMapping<'a, C>,
}

impl<'a, C> StakingContext<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
{
    pub fn new(sc_ref: &'a C, payment_token_id: &TokenIdentifier<C::Api>) -> Self {
        let caller = sc_ref.blockchain().get_caller();
        let staking_module_type = sc_ref.stake_pool_type_configuration(payment_token_id).get();

        let aggregated_score = sc_ref
            .aggregated_staking_score(&StakingModuleType::All)
            .get();
        let aggregated_user_score = match sc_ref
            .aggregated_user_staking_score(&StakingModuleType::All, &caller)
            .is_empty()
        {
            true => BigUint::zero(),
            false => sc_ref
                .aggregated_user_staking_score(&StakingModuleType::All, &caller)
                .get(),
        };
        let secondary_aggregated_score =
            sc_ref.aggregated_staking_score(&staking_module_type).get();
        let secondary_aggregated_user_score = match sc_ref
            .aggregated_user_staking_score(&staking_module_type, &caller)
            .is_empty()
        {
            true => BigUint::zero(),
            false => sc_ref
                .aggregated_user_staking_score(&staking_module_type, &caller)
                .get(),
        };

        let staking_module_impl =
            staking_module_type.get_module(sc_ref, payment_token_id.clone(), caller.clone());
        Self {
            sc_ref,
            caller,
            aggregated_score,
            aggregated_user_score,
            secondary_aggregated_score,
            secondary_aggregated_user_score,
            staking_module_type,
            staking_module_impl,
        }
    }

    pub fn add_to_stake(&self, payments: &ManagedVec<C::Api, EsdtTokenPayment<C::Api>>) {
        self.secure_rewards();

        for payment in payments.iter() {
            self.staking_module_impl
                .add_to_storage(payment.token_nonce, payment.amount);
        }

        self.update_primary_score();
        self.update_secondary_score();
    }

    pub fn start_unbonding(&self, payload: StartUnbondingPayload<C::Api>) -> bool {
        self.secure_rewards();

        let unbonding_result = self.staking_module_impl.start_unbonding(payload.clone());
        if unbonding_result {
            self.sc_ref
                .unbonding_assets(&self.caller)
                .insert(self.sc_ref.blockchain().get_block_timestamp(), payload);
            self.update_primary_score();
            self.update_secondary_score();
        }

        unbonding_result
    }

    fn secure_rewards(&self) {
        for reward_token_id in self.sc_ref.reward_token_identifiers().iter() {
            secure_rewards(
                self.sc_ref,
                &self.caller,
                &reward_token_id,
                &self.staking_module_type,
            );
        }
    }

    fn update_primary_score(&self) {
        let new_user_score = self.staking_module_impl.get_final_user_score();

        let new_aggregated_score =
            &self.aggregated_score + &new_user_score - &self.aggregated_user_score;

        self.sc_ref
            .aggregated_user_staking_score(&StakingModuleType::All, &self.caller)
            .set(new_user_score);
        self.sc_ref
            .aggregated_staking_score(&StakingModuleType::All)
            .set(new_aggregated_score);
    }

    fn update_secondary_score(&self) {
        let new_user_score = self.staking_module_impl.get_final_secondary_score();

        let new_aggregated_score = &self.secondary_aggregated_score
            - &self.secondary_aggregated_user_score
            + &new_user_score;

        self.sc_ref
            .aggregated_user_staking_score(&self.staking_module_type, &self.caller)
            .set(new_user_score);
        self.sc_ref
            .aggregated_staking_score(&self.staking_module_type)
            .set(new_aggregated_score);
    }
}
