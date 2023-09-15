# NFT Staking

This project is a Non-Fungible Token (NFT) staking system. It allows users to stake their NFTs and earn rewards based on the staking score of their NFTs. The system supports multiple staking pools, each with its own staking module type and reward distribution.

## Features

- Staking: Users can stake their NFTs in the staking pool. The staking score is determined by the staking module type of the pool. The system supports staking of multiple NFTs at once.

- Reward Distribution: The system distributes rewards to the stakers based on their staking score. The reward distribution can be done for all staking pools or for a specific staking pool.

- Claiming Rewards: Users can claim their rewards from the system. The rewards are distributed based on the staking score of the user's staked NFTs.

# Usage

## Setting up a pool

Here are the steps to setup and configure a staking pool in the NFT staking system:

### Configure the staking pool type

Each token identifier has a 1:1 mapping with a staking pool type.
One can register a new mapping using the following endpoint:

```rust
#[only_owner]
#[endpoint(createPool)]
fn register_new_staking_pool(
    &self,
    collection_token_identifier: TokenIdentifier,
    staking_module_type: StakingModuleType,
)
```

### Configure NFT/SFT scores

Each NFT/SFT score can be granularly set using the score system:

- `base_asset_score`: a default score used for each nonce of the collection
- `nonce_asset_score`: a nonce specific score used to override the `base_asset_score`

The collection score can be configured using the following endpoints:

```rust
#[only_owner]
#[endpoint(setBaseAssetScore)]
fn set_base_asset_score(
    &self,
    collection_token_identifier: &TokenIdentifier,
    staking_module: &StakingModuleType,
    score: usize,
)
```

```rust
#[only_owner]
#[endpoint(setNonceAssetScore)]
fn set_nonce_asset_score(
    &self,
    collection_token_identifier: &TokenIdentifier,
    staking_module: &StakingModuleType,
    score: usize,
    nonces: MultiValueEncoded<u64>,
)
```

Each of the above endpoints expects both the collection token identifier and the staking module, although the staking module type was already configured during the prior step.

This is happening because an NFT/SFT collection can have one or more different scores based on the distributed staking reward.

**Important notice:** in order for a collection to benefit from the primary reward (distributed to the whole staking system) it must have a scored defined for the `StakingModuleType::All` staking module.

### Secondary level rewards

The secondary level rewards are all rewards that will be distributed to a specific staking pool.

In order to support this feature, each reward token must be configured in the staking system and mapped to the corresponding `StakingModuleType`s. The mapping relationship is 1 to many (1..\*), which means one can distribute the same reward token to multiple pools as well as multiple tokens to the same pool.

Registering a token can be done using the following endpoint:

```rust
#[only_owner]
#[endpoint(registerRewardToken)]
fn register_reward_token(
    &self,
    reward_token_identifier: TokenIdentifier,
    staking_module_type: StakingModuleType,
)
```

## Staking

Users can stake their NFTs using the stake function. The function expects NFT/SFT transfers.

**Limitations:** The user can only send **one NFT collection** per transfer. This means the user can stake as many NFTs as needed but the items of each NFT collection must be sent in a new transaction. The user can, however, stake multiple NFTs/SFTs in the same transaction as long as they all have the same `token_identifier`.

```rust
#[payable("*")]
#[endpoint(stake)]
fn stake(&self)
```

Each staked NFT or SFT will trigger a pending reward commit (to not affect the previously earned rewards with the stake change) and a score recalculation.

Each NFT/SFT collection must have their score values configured in order to be eligible for staking and earning rewards.

### Upcoming Staking Improvements

For supporting the currently staked SFTs, new stake endpoints will be added thus migrating the existing mechanisms to use this NFT staking system.

## Reward Distribution

The system distributes rewards using the distribute_reward function. This function expects a single ESDT token payment transfer.
The `reward_rate` each staked point will receive is computed as follows: `transferred amount / aggregated score` for `StakingModuleType::All`.

```rust
#[payable("*")]
#[endpoint(distributeGeneralReward)]
fn distribute_reward(&self)
```

For distributing rewards for a specific staking pool, the distribute_secondary_reward function is used. The function takes the reward token ID, staking pool token ID, and total reward amount as input.

```rust
#[payable("*")]
#[endpoint(distributeSecondaryReward)]
fn distribute_secondary_reward(&self, target: TokenIdentifier)
```

# Live configurations

(Work in progress)
This section is dedicated to breaking down the configurations DemiourgosHoldings will support within the NFT Staking System.

## Snake SFTs

This collection will only yield rewards in the secondary reward distribution mechanism and 0 rewards from the primary reward distribution mechanism.

- module type: `StakingModuleType::SnakesSfts`
- `base_asset_score(StakingModuleType::All)` set to 0
- `base_asset_score(StakingModuleType::SnakesSfts)` set to 10000

## Coding division SFTs

This collection will yield primary rewards and, with the configuration below, will receive no secondary rewards.
Each full set (of 10 pieces) will yield an extra 25 points for the staker.

- module type: `StakingModuleType::CodingDivisionSfts`
- `base_asset_score(StakingModuleType::All)` set to 5
- `full_set_bonus_score(StakingModuleType::All)` set to 25

## VestaX DAO SFTs

NOT CONFIGURED YET - awaiting confirmation

## Bloodshed NFTs

This collection will yield both primary and secondary rewards.

- module type: `StakingModuleType::Bloodshed`
- `base_asset_score(StakingModuleType::All)` set to 1
- `nonce_asset_score(StakingModuleType::All)` set as follows:
  - Rare Bloodshed NFT: 3 Points
  - Epic Bloodshed NFT: 4 Points
  - Legendary Bloodshed NFT: 11 Points
- `base_asset_score(StakingModuleType::Bloodshed)` set to TBD
- `nonce_asset_score(StakingModuleType::Bloodshed)` set to TBD

## XBunnies NFTs

This collection will yield both primary and secondary rewards.

- module type: `StakingModuleType::XBunnies`
- `base_asset_score(StakingModuleType::All)` set to 2
- `nonce_asset_score(StakingModuleType::All)` set to 160 for legendary NFTs
- Staking module specific scores to be set according to [the holy GitBook](https://demiourgos-holdings-tm.gitbook.io/demiourgos-library/v/nft-staking/xbunnies-nfts).

## Nosferatu NFTs

Similar configuration to XBunnies and Bloodshed NFTs, details in [the GitBook](https://demiourgos-holdings-tm.gitbook.io/demiourgos-library/v/nft-staking/xbunnies-nfts).

## Seasonal VAULT Booster SFTs

NOT CONFIGURED YET - awaiting confirmation

# Other configuration possibilities

The whole logic can be used however and can support multiple scenarios that have not been defined here.

The logic breakdown is that one can configure:

- what algorithm should be used for computing the scores
- what reward is a staked NFT/SFT collection eligible for
- what share of the distributed reward each collection is eligible for
- different reward tokens for secondary reward distribution
- same/multiple token(s) for each/some/all staking pools
- and more..

# Scripts usage

## One time setup

- Open a terminal and go to the scripts folder
- Run `npm i`
- Create the `.env` file (you can copy paste the `.env.default` file, rename it and fill it with correct info) and update it accordingly

## Script

Running `node index.js` should present a set of options, such as:

```
Please choose an option:
 1. Deploy
 2. Upgrade
 3. Add Pool
```

Each of these options are being automatically configured based on the `.env` file.

Each deployment will override the `config.json` entry of the target `.env ENVIRONMENT` variable.

Each upgrade will look for the pre-existing address in `config.json` based on the target `ENVIRONMENT` variable.

### Deploy

Deploys a new instance of the contract using the PEM file and `ENVIRONMENT` settings provided in `.env`. Will store the new address in `config.json` upon successful execution.

### Upgrade

Upgrades an existing instance of the contract using the same procedure explained above.

### Commit Pools Data

Will look in the `config.json` for uncomitted pools. Once a pool has been setup, it will be market as committed and ignored for further updates. If a pool must be updated, remove the "isCommitted" property and run the script again.

#### Pool Settings that must be provided:

- `name`: Ignored by the script, useful for keeping track of which pool does what
- `collectionTokenIdentifier`: The NFT/SFT collection token identifier of the pool
- `stakingModuleType`: One of the following values
  - CodingDivisionSfts
  - XBunnies
  - Bloodshed
  - Nosferatu
  - VestaXDAO
  - SnakesSfts
- `scoreConfiguration`: the scores based on which the reward is being distributed. Example of configuration:

```json
{
    ...
    "scoreConfiguration": {
          "All": {
            "base": 10,
            "granular": []
          },
          "XBunnies": {
            "base": 5,
            "granular": [
                {
                    "nonces": [10, 11, 12],
                    "score": 10
                },
                {
                    "nonces": [210, 211, 212],
                    "score": 15
                },
                {
                    "nonces": [310, 311, 312, 313],
                    "score": 20
                },
            ]
          }
        }
    ...
}
```

#### Reward tokens

Each reward token must be registered as a pool reward if it's going to be distributed to a specific pool.
These tokens can be set under `"poolSettings[ENVIRONMENT][rewardTokens]"` as key-value pairs. Example:

```json
{
  ...
  "rewardTokens": [
    {
      "tokenIdentifier": "REWARDTKN-123456",
      "stakingModuleType": "XBunnies"
    }
  ]
  ...
}
```

The same `"isCommitted"` logic is applied here as well.
