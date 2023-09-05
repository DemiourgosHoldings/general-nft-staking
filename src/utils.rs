multiversx_sc::imports!();

pub fn get_unstored_pending_rewards<'a, C>(
    sc_ref: &'a C,
    address: &ManagedAddress<C::Api>,
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
        if sc_ref.reward_rate(epochs).is_empty() {
            continue;
        }
        let reward_rate = sc_ref.reward_rate(epochs).get();
        pending_reward += &user_score * &reward_rate;
    }

    pending_reward
}
