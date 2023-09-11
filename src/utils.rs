use crate::staking_modules::staking_module_type::StakingModuleType;

multiversx_sc::imports!();

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

    for token_id in sc_ref.reward_token_identifiers().iter() {
        for staking_module_type in sc_ref.reward_token_to_staking_module_map(&token_id).iter() {
            match get_token_pending_reward_payment(
                sc_ref,
                address,
                &token_id,
                store_rewards,
                &staking_module_type,
            ) {
                Some(pending_reward) => pending_rewards.push(pending_reward),
                None => continue,
            };
        }
    }

    pending_rewards
}

pub fn get_token_pending_reward_payment<'a, C>(
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
        get_total_token_pending_reward(sc_ref, address, &token_identifier, &staking_module_type);
    if &pending_reward == &0 {
        return None;
    }

    if store_rewards {
        secure_rewards(sc_ref, address, &token_identifier, &staking_module_type);
    }

    Some(EsdtTokenPayment::new(
        token_identifier.clone(),
        0,
        pending_reward,
    ))
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
    let last_claimed_epoch = sc_ref.last_claimed_epoch(staking_module, address).get();
    let current_epoch = sc_ref.blockchain().get_block_epoch();
    let user_score = sc_ref
        .aggregated_user_staking_score(staking_module, address)
        .get();

    let mut pending_reward = BigUint::zero();
    for current_epoch in last_claimed_epoch + 1..=current_epoch {
        if sc_ref
            .reward_rate(current_epoch, token_identifier)
            .is_empty()
        {
            continue;
        }
        let reward_rate = sc_ref.reward_rate(current_epoch, token_identifier).get();
        pending_reward += &user_score * &reward_rate;
    }

    pending_reward
}

pub fn secure_rewards<'a, C>(
    sc_ref: &'a C,
    address: &ManagedAddress<C::Api>,
    token_identifier: &TokenIdentifier<C::Api>,
    staking_module: &StakingModuleType,
) where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::user_data::UserDataStorageModule,
    C: crate::storage::score::ScoreStorageModule,
{
    let pending_unstored_rewards =
        get_unstored_pending_rewards(sc_ref, address, token_identifier, staking_module);
    let stored_rewards = match sc_ref.pending_rewards(address, token_identifier).is_empty() {
        true => BigUint::zero(),
        false => sc_ref.pending_rewards(address, token_identifier).get(),
    };

    let block_epoch = sc_ref.blockchain().get_block_epoch();
    if sc_ref.reward_rate(block_epoch, token_identifier).is_empty() {
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
        .set(pending_unstored_rewards + stored_rewards);
}

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

pub fn apply_new_deb<C>(
    old_general_aggregated_score: &BigUint<C::Api>,
    old_user_score: &BigUint<C::Api>,
    old_deb: &BigUint<C::Api>,
    new_deb: &BigUint<C::Api>,
    deb_denomination: &BigUint<C::Api>,
) -> (BigUint<C::Api>, BigUint<C::Api>)
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::user_data::UserDataStorageModule,
    C: crate::storage::score::ScoreStorageModule,
{
    let score_without_deb = &(old_user_score * deb_denomination) / old_deb;
    let new_user_score = &(score_without_deb * new_deb) / deb_denomination;
    let new_general_aggregated_score =
        old_general_aggregated_score - old_user_score + &new_user_score;

    (new_user_score, new_general_aggregated_score)
}
