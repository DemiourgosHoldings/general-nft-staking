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
