use crate::staking_modules::staking_module_type::StakingModuleType;

multiversx_sc::imports!();

pub fn claim_all_pending_rewards<'a, C>(
    sc_ref: &'a C,
    caller: &ManagedAddress<C::Api>,
) -> ManagedVec<C::Api, EsdtTokenPayment<C::Api>>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::user_data::UserDataStorageModule,
    C: crate::storage::score::ScoreStorageModule,
{
    let pending_rewards = get_all_pending_rewards(sc_ref, caller, true);

    for pending_reward in pending_rewards.iter() {
        sc_ref
            .pending_rewards(caller, &pending_reward.token_identifier)
            .clear();
    }

    pending_rewards
}

pub fn get_all_pending_rewards<'a, C>(
    sc_ref: &'a C,
    address: &ManagedAddress<C::Api>,
    store_rewards: bool,
) -> ManagedVec<C::Api, EsdtTokenPayment<C::Api>>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::user_data::UserDataStorageModule,
    C: crate::storage::score::ScoreStorageModule,
{
    let mut pending_rewards = ManagedVec::new();

    for (token_identifier, staking_module_type) in sc_ref.reward_token_id_mapping().iter() {
        if let Some(pending_reward) = get_single_token_pending_reward_payment(
            sc_ref,
            address,
            &token_identifier,
            store_rewards,
            &staking_module_type,
        ) {
            pending_rewards.push(pending_reward);
        }
    }

    pending_rewards
}

pub fn get_single_token_pending_reward_payment<'a, C>(
    sc_ref: &'a C,
    address: &ManagedAddress<C::Api>,
    token_identifier: &TokenIdentifier<C::Api>,
    store_rewards: bool,
    staking_module_type: &StakingModuleType,
) -> Option<EsdtTokenPayment<C::Api>>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::user_data::UserDataStorageModule,
    C: crate::storage::score::ScoreStorageModule,
{
    let pending_reward =
        get_total_token_pending_reward(sc_ref, address, token_identifier, staking_module_type);
    if &pending_reward == &0 {
        return None;
    }

    if store_rewards {
        secure_rewards(
            sc_ref,
            address,
            &token_identifier,
            &staking_module_type,
            Some(pending_reward.clone()),
        );
    }

    Some(EsdtTokenPayment::new(
        token_identifier.clone(),
        0,
        pending_reward,
    ))
}

pub fn secure_rewards<'a, C>(
    sc_ref: &'a C,
    address: &ManagedAddress<C::Api>,
    token_identifier: &TokenIdentifier<C::Api>,
    staking_module: &StakingModuleType,
    pending_rewards_opt: Option<BigUint<C::Api>>,
) where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::user_data::UserDataStorageModule,
    C: crate::storage::score::ScoreStorageModule,
{
    let pending_rewards = match pending_rewards_opt {
        Some(val) => val,
        None => get_total_token_pending_reward(sc_ref, address, token_identifier, staking_module),
    };
    let block_epoch = sc_ref.blockchain().get_block_epoch();
    if sc_ref
        .reward_rate(block_epoch, staking_module, token_identifier)
        .is_empty()
    {
        sc_ref
            .last_claimed_epoch(staking_module, address)
            .set(&block_epoch - 1);
    } else {
        sc_ref
            .last_claimed_epoch(staking_module, address)
            .set(block_epoch);
    }

    sc_ref
        .pending_rewards(address, token_identifier)
        .update(|old_value| *old_value = pending_rewards);
}

pub fn get_total_token_pending_reward<'a, C>(
    sc_ref: &'a C,
    address: &ManagedAddress<C::Api>,
    token_identifier: &TokenIdentifier<C::Api>,
    staking_module: &StakingModuleType,
) -> BigUint<C::Api>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::user_data::UserDataStorageModule,
    C: crate::storage::score::ScoreStorageModule,
{
    let not_stored_amount =
        get_unstored_pending_rewards(sc_ref, address, token_identifier, staking_module);
    let stored_amount = sc_ref.pending_rewards(address, token_identifier).get();

    not_stored_amount + stored_amount
}

pub fn get_unstored_pending_rewards<'a, C>(
    sc_ref: &'a C,
    address: &ManagedAddress<C::Api>,
    token_identifier: &TokenIdentifier<C::Api>,
    staking_module: &StakingModuleType,
) -> BigUint<C::Api>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::user_data::UserDataStorageModule,
    C: crate::storage::score::ScoreStorageModule,
{
    let user_score = sc_ref
        .aggregated_user_staking_score(staking_module, address)
        .get();

    if &user_score == &0 {
        return BigUint::zero();
    }

    let last_claimed_epoch = sc_ref.last_claimed_epoch(staking_module, address).get();
    let current_epoch = sc_ref.blockchain().get_block_epoch();

    let mut pending_reward = BigUint::zero();
    for current_epoch in last_claimed_epoch + 1..=current_epoch {
        if sc_ref
            .reward_rate(current_epoch, staking_module, token_identifier)
            .is_empty()
        {
            continue;
        }
        let reward_rate = sc_ref
            .reward_rate(current_epoch, staking_module, token_identifier)
            .get();
        pending_reward += &user_score * &reward_rate;
    }

    pending_reward
}
