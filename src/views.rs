use crate::{
    staking_modules::staking_module_type::StakingModuleType,
    types::ui_types::{
        UIAggregatedPoolScore, UIExtendedAggregatedPoolScore, UIUnbondingAsset, UIUserDataPayload,
        UIUserPoolData,
    },
    utils::get_all_pending_rewards,
};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ViewsModule:
    crate::storage::config::ConfigModule
    + crate::storage::user_data::UserDataStorageModule
    + crate::storage::score::ScoreStorageModule
{
    #[view(getGeneralStakingData)]
    fn get_general_staking_data(&self) -> ManagedVec<UIAggregatedPoolScore<Self::Api>> {
        let mut staking_data = ManagedVec::new();

        for pool_type in StakingModuleType::iter() {
            let general_score = self.aggregated_staking_score(&pool_type).get();
            staking_data.push(UIAggregatedPoolScore {
                pool_type: (*pool_type).clone() as u8,
                pool_score: general_score,
            });
        }

        staking_data
    }

    #[view(getUserStakingData)]
    fn get_user_staking_data(&self, address: ManagedAddress) -> UIUserDataPayload<Self::Api> {
        let user_deb = self.user_deb(&address).get();

        let store_pending_rewards = false;
        let pending_rewards = get_all_pending_rewards(self, &address, store_pending_rewards);

        let mut user_pool_data = ManagedVec::new();
        for token_id in self.eligible_stake_token_identifiers().iter() {
            let current_pool_data = self.get_user_pool_data(&address, &token_id);
            if current_pool_data.is_none() {
                continue;
            }
            user_pool_data.push(current_pool_data.unwrap());
        }

        let unbonding_assets = self.parse_unbonding_assets(&address);

        UIUserDataPayload {
            pending_rewards,
            user_pool_data,
            unbonding_assets,
            user_deb,
        }
    }

    #[view(getUserPoolStakingData)]
    fn get_user_pool_data(
        &self,
        address: &ManagedAddress,
        token_identifier: &TokenIdentifier<Self::Api>,
    ) -> Option<UIUserPoolData<Self::Api>> {
        let staked_nfts_opt = self.staked_nfts(&token_identifier).get(&address);
        if staked_nfts_opt.is_none() {
            return Option::None;
        }

        let pool_staked_assets = staked_nfts_opt.unwrap();

        let pool_module_type = self.stake_pool_type_configuration(token_identifier).get();
        let pool_score = self
            .aggregated_user_staking_score(&pool_module_type, address)
            .get();
        let raw_pool_score = self
            .raw_aggregated_user_staking_score(&pool_module_type, address)
            .get();

        let pool_score_data = UIExtendedAggregatedPoolScore {
            pool_type: pool_module_type as u8,
            pool_score,
            raw_pool_score,
        };

        let user_pool_data = UIUserPoolData {
            pool_token_identifier: token_identifier.clone(),
            pool_score_data,
            pool_staked_assets,
        };

        Option::Some(user_pool_data)
    }

    fn parse_unbonding_assets(
        &self,
        address: &ManagedAddress,
    ) -> ManagedVec<UIUnbondingAsset<Self::Api>> {
        let mut unbonding_assets = ManagedVec::new();
        for key in self.unbonding_assets(address).keys() {
            let unbonding_payload = self.unbonding_assets(address).get(&key).unwrap();
            unbonding_assets.push(UIUnbondingAsset {
                timestamp: key,
                assets: unbonding_payload,
            })
        }

        unbonding_assets
    }
}
