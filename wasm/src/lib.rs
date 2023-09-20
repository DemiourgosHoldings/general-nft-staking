// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           38
// Async Callback (empty):               1
// Total number of exported functions:  40

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    nft_staking
    (
        init => init
        stake => stake
        startUnbonding => start_unbonding
        claimUnbonded => claim_unbonded
        claimRewards => claim_rewards
        getPendingReward => get_pending_reward
        getStakingPoolTypeConfiguration => stake_pool_type_configuration
        getStakingModulesByRewardToken => reward_token_to_staking_module_map
        getUnbondingTimePenalty => unbonding_time_penalty
        getRewardTokenIdentifiers => reward_token_identifiers
        getPrimaryRewardTokenIdentifier => primary_reward_token_identifier
        getEligibleStakeTokenIdentifiers => eligible_stake_token_identifiers
        getBaseAssetScore => base_asset_score
        getNonceAssetScore => nonce_asset_score
        getFullSetScore => full_set_score
        getStakedNfts => staked_nfts
        getUnbondingAssets => unbonding_assets
        getUserDeb => user_deb
        getRawAggregatedUserStakingScore => raw_aggregated_user_staking_score
        getAggregatedStakingScore => aggregated_staking_score
        getAggregatedUserStakingScore => aggregated_user_staking_score
        getPendingRewards => pending_rewards
        getLastClaimedEpoch => last_claimed_epoch
        getRewardRate => reward_rate
        getRewardDistributionTimestamp => reward_distribution_timestamp
        distributeGeneralReward => distribute_reward
        distributeSecondaryReward => distribute_secondary_reward
        distributeCompanyShareReward => distribute_company_share_reward
        updateDeb => update_deb
        createPool => register_new_staking_pool
        overridePoolType => override_stake_pool_type
        setBaseAssetScore => set_base_asset_score
        setNonceAssetScore => set_nonce_asset_score
        setNonceAssetScoreByRange => set_nonce_asset_score_by_range
        registerRewardToken => register_reward_token
        setFullSetScore => set_full_set_score
        getGeneralStakingData => get_general_staking_data
        getUserStakingData => get_user_staking_data
        getUserPoolStakingData => get_user_pool_data
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
