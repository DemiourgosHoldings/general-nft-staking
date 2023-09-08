multiversx_sc::imports!();

pub fn get_unstored_pending_rewards<'a, C>(
    sc_ref: &'a C,
    address: &ManagedAddress<C::Api>,
    token_identifier: &TokenIdentifier<C::Api>,
) -> BigUint<C::Api>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::user_data::UserDataStorageModule,
    C: crate::storage::score::ScoreStorageModule,
{
    let last_claimed_epoch = sc_ref.last_claimed_epoch(address).get();
    let current_epoch = sc_ref.blockchain().get_block_epoch();
    let user_score = sc_ref.aggregated_user_staking_score(address).get();

    let mut pending_reward = BigUint::zero();
    for epochs in last_claimed_epoch + 1..=current_epoch {
        if sc_ref.reward_rate(epochs, token_identifier).is_empty() {
            continue;
        }
        let reward_rate = sc_ref.reward_rate(epochs, token_identifier).get();
        pending_reward += &user_score * &reward_rate;
    }

    pending_reward
}

pub fn secure_rewards<'a, C>(
    sc_ref: &'a C,
    address: &ManagedAddress<C::Api>,
    token_identifier: &TokenIdentifier<C::Api>,
) where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::user_data::UserDataStorageModule,
    C: crate::storage::score::ScoreStorageModule,
{
    let pending_unstored_rewards = get_unstored_pending_rewards(sc_ref, address, token_identifier);
    let stored_rewards = match sc_ref.pending_rewards(address, token_identifier).is_empty() {
        true => BigUint::zero(),
        false => sc_ref.pending_rewards(address, token_identifier).get(),
    };

    let block_epoch = sc_ref.blockchain().get_block_epoch();
    if sc_ref.reward_rate(block_epoch, token_identifier).is_empty() {
        sc_ref.last_claimed_epoch(address).set(&block_epoch - 1);
    } else {
        sc_ref.last_claimed_epoch(address).set(block_epoch);
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
    let base_reward_opt = claim_single_token_pending_rewards(
        sc_ref,
        caller,
        sc_ref.primary_reward_token_identifier().get(),
    );
    let mut pending_rewards = match base_reward_opt {
        Some(base_reward) => ManagedVec::from_single_item(base_reward),
        None => ManagedVec::new(),
    };
    for token_id in sc_ref.secondary_reward_token_identifiers().iter() {
        let reward_opt = claim_single_token_pending_rewards(sc_ref, caller, token_id);
        if reward_opt.is_none() {
            continue;
        }
        pending_rewards.push(reward_opt.unwrap());
    }

    pending_rewards
}

pub fn claim_single_token_pending_rewards<'a, C>(
    sc_ref: &'a C,
    caller: &ManagedAddress<C::Api>,
    token_identifier: TokenIdentifier<C::Api>,
) -> Option<EsdtTokenPayment<C::Api>>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::user_data::UserDataStorageModule,
    C: crate::storage::score::ScoreStorageModule,
{
    secure_rewards(sc_ref, caller, &token_identifier);

    let pending_reward = sc_ref.pending_rewards(&caller, &token_identifier).get();
    if &pending_reward == &BigUint::zero() {
        return Option::None;
    }

    sc_ref.pending_rewards(&caller, &token_identifier).clear();
    let payment = EsdtTokenPayment::new(token_identifier, 0, pending_reward);

    Option::Some(payment)
}
