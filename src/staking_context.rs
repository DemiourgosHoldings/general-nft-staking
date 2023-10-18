use crate::{
    constants::DEB_DENOMINATION,
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
    user_deb: BigUint<C::Api>,
    aggregated_general_score: BigUint<C::Api>,
    aggregated_user_score_with_deb: BigUint<C::Api>,
    secondary_aggregated_general_score: BigUint<C::Api>,
    secondary_aggregated_user_score_with_deb: BigUint<C::Api>,
    initial_pool_user_score: BigUint<C::Api>,
    secondary_initial_pool_user_score: BigUint<C::Api>,
    staking_module_type: StakingModuleType,
    staking_module_impl: StakingModuleTypeMapping<'a, C>,
    token_identifier: TokenIdentifier<C::Api>,
}

impl<'a, C> StakingContext<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
    C: crate::storage::user_data::UserDataStorageModule,
    C: crate::requirements::RequirementsModule,
{
    pub fn new(sc_ref: &'a C, payment_token_id: &TokenIdentifier<C::Api>) -> Self {
        let caller = sc_ref.blockchain().get_caller();
        Self::new_with_address(sc_ref, payment_token_id, caller)
    }

    pub fn new_with_address(
        sc_ref: &'a C,
        payment_token_id: &TokenIdentifier<C::Api>,
        caller: ManagedAddress<C::Api>,
    ) -> Self {
        let staking_module_type = sc_ref.stake_pool_type_configuration(payment_token_id).get();

        let (aggregated_general_score, aggregated_user_score_with_deb) =
            Self::get_score_data(&sc_ref, &StakingModuleType::All, &caller);
        let (secondary_aggregated_general_score, secondary_aggregated_user_score_with_deb) =
            Self::get_score_data(&sc_ref, &staking_module_type, &caller);

        let staking_module_impl =
            staking_module_type.get_module(sc_ref, payment_token_id.clone(), caller.clone());

        let user_deb = sc_ref.user_deb(&caller).get();
        let initial_pool_user_score = Self::apply_deb(
            &staking_module_impl.get_base_user_score(&StakingModuleType::All),
            &user_deb,
        );

        let secondary_initial_pool_user_score =
            staking_module_impl.get_base_user_score(&staking_module_type);

        Self {
            sc_ref,
            caller,
            user_deb,
            aggregated_general_score,
            aggregated_user_score_with_deb,
            secondary_aggregated_general_score,
            secondary_aggregated_user_score_with_deb,
            initial_pool_user_score,
            secondary_initial_pool_user_score,
            staking_module_type,
            staking_module_impl,
            token_identifier: payment_token_id.clone(),
        }
    }

    pub fn add_to_stake(&mut self, payments: &ManagedVec<C::Api, EsdtTokenPayment<C::Api>>) {
        self.secure_all_rewards();
        for payment in payments.iter() {
            self.sc_ref
                .require_token_matches(&self.token_identifier, &payment.token_identifier);

            self.staking_module_impl
                .add_to_storage(payment.token_nonce, payment.amount);
        }
    }

    pub fn start_unbonding(&mut self, payload: StartUnbondingPayload<C::Api>) -> bool {
        self.secure_all_rewards();

        let unbonding_result = self.staking_module_impl.start_unbonding(payload.clone());
        if unbonding_result {
            self.sc_ref
                .unbonding_assets(&self.caller)
                .insert(self.sc_ref.blockchain().get_block_timestamp(), payload);
        }

        unbonding_result
    }

    fn secure_all_rewards(&self) {
        for reward_token_id in self.sc_ref.reward_token_identifiers().iter() {
            secure_rewards(
                self.sc_ref,
                &self.caller,
                &reward_token_id,
                &self.staking_module_type,
                None,
            );
        }
    }

    fn update_primary_score(&self) {
        self.update_score(
            &StakingModuleType::All,
            &self.initial_pool_user_score,
            &self.aggregated_user_score_with_deb,
            &self.aggregated_general_score,
        );
    }

    fn update_secondary_score(&self) {
        self.update_score(
            &self.staking_module_type,
            &self.secondary_initial_pool_user_score,
            &self.secondary_aggregated_user_score_with_deb,
            &self.secondary_aggregated_general_score,
        );
    }

    fn update_score(
        &self,
        module_type: &StakingModuleType,
        initial_pool_user_score: &BigUint<C::Api>,
        aggregated_user_score_with_deb: &BigUint<C::Api>,
        aggregated_general_score: &BigUint<C::Api>,
    ) {
        let new_base_user_score = self.staking_module_impl.get_base_user_score(module_type);
        let new_pool_user_score = Self::apply_deb(&new_base_user_score, &self.user_deb);
        if &new_pool_user_score == initial_pool_user_score {
            return;
        }

        let new_aggregated_general_score =
            aggregated_general_score + &new_pool_user_score - initial_pool_user_score;

        let new_user_score =
            aggregated_user_score_with_deb + &new_pool_user_score - initial_pool_user_score;

        self.sc_ref
            .aggregated_user_staking_score(module_type, &self.caller)
            .set(new_user_score);
        self.sc_ref
            .aggregated_staking_score(module_type)
            .set(new_aggregated_general_score);
        self.sc_ref
            .raw_aggregated_user_staking_score(module_type, &self.caller)
            .set(&new_base_user_score);
    }

    fn get_score_data(
        sc_ref: &'a C,
        staking_module_type: &StakingModuleType,
        caller: &ManagedAddress<C::Api>,
    ) -> (BigUint<C::Api>, BigUint<C::Api>) {
        let aggregated_general_score = sc_ref.aggregated_staking_score(staking_module_type).get();
        let aggregated_user_score_with_deb = match sc_ref
            .aggregated_user_staking_score(staking_module_type, caller)
            .is_empty()
        {
            true => BigUint::zero(),
            false => sc_ref
                .aggregated_user_staking_score(staking_module_type, caller)
                .get(),
        };

        (aggregated_general_score, aggregated_user_score_with_deb)
    }

    fn apply_deb(user_score: &BigUint<C::Api>, deb: &BigUint<C::Api>) -> BigUint<C::Api> {
        let deb_denomination = BigUint::from(DEB_DENOMINATION);

        if deb <= &deb_denomination {
            return user_score.clone();
        }

        user_score * deb / deb_denomination
    }

    pub fn update_staking_scores(&mut self) {
        self.update_primary_score();
        self.update_secondary_score();
    }

    pub fn get_base_user_score(&self, staking_module_type: &StakingModuleType) -> BigUint<C::Api> {
        self.staking_module_impl
            .get_base_user_score(staking_module_type)
    }
}
