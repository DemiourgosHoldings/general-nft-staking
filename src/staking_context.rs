use crate::staking_modules::staking_module_type::{StakingModuleTypeFactory, VestaStakingModule};

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
}

impl<'a, C> StakingContext<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    pub fn new(sc_ref: &'a C) -> Self {
        let caller = sc_ref.blockchain().get_caller();
        let aggregated_score = sc_ref.aggregated_staking_score().get();
        let aggregated_user_score = match sc_ref.aggregated_user_staking_score(&caller).is_empty() {
            true => BigUint::zero(),
            false => sc_ref.aggregated_user_staking_score(&caller).get(),
        };

        Self {
            sc_ref,
            aggregated_score,
            aggregated_user_score,
            caller,
        }
    }

    pub fn add_to_stake(&self) {
        self.secure_rewards();

        let payments = self.sc_ref.call_value().all_esdt_transfers();
        for payment in payments.iter() {
            self.process_stake_payment(payment);
        }
    }

    fn process_stake_payment(&self, payment: EsdtTokenPayment<C::Api>) {
        let staking_module_type = self
            .sc_ref
            .stake_pool_type_configuration(&payment.token_identifier)
            .get();
        let module_impl = staking_module_type.get_module(
            self.sc_ref,
            payment.token_identifier,
            self.caller.clone(),
        );
        module_impl.add_to_storage(payment.token_nonce, payment.amount);
    }

    fn secure_rewards(&self) {}

    fn compute_rewards(&self) -> BigUint<C::Api> {
        BigUint::zero()
    }
}
