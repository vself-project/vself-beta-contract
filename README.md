## vSelf Beta Event Management Contract

Current repository contains backend source code and tooling which was developed during NEAR Metabuild Hackathon.

### Event Lifecycle (User story)

1. Event is initiated and setup through web app interface
1. Event statistics and data on checkins/consent is being aggregated (scanning QR on mobile)
1. Reward is minted and distributed to client acccount (upon successful action)
1. Event is finished either when time is due or if abort method is called

### Data Layer

1. EventData - current event metadata
1. EventStats - current event aggregated statistics
1. EventActions - current event list of user actions (e.g. checkins)
1. PastEvents - collection of past events (historical data)

Contract serves as a registry for NFT tokens and implements NEAR standarts.

### Deployment

1. testnet => beta_v8.ilerik.testnet
1. mainnet => No mainnet for now. Current version is not abuse proof.

### Development

1. Clone repository and switch directory
```bash
git clone <repoUrl>
cd vself-beta-contract
```
1. Install dependencies
    ```bash
    yarn
    ```
1. Build contract
    ```bash
    yarn build
    ```
1. Deploy contract to dev account
    ```bash
    yarn dev:deploy
    ```
1. Test contract deployed to dev account
    ```bash
    yarn test:contract
    ```

### Contract Interface

Change Methods:

1. start_event()
1. stop_event()
1. checkin()

Read-only Methods:

1. get_event_data()
1. get_event_actions()
1. get_event_stats()

### Availibility API server

Deployed to google cloud, has contract level account credentials for now.

HTTPS Endpoints:
1. /status -> Num (number of quests) / 0 - for no event
1. /rewards?nearid=<blabla> -> [ string ] (url of NFT images)
1. /balance?nearid=<account_id> -> {[Reward]}
1. /checkin?nearid=<account_id>&qr=<string> -> 
    OnSucces: -> Reward: { index, got, title, description }
    OnFail  : -> Reward: { -1, false, "nothing", "nothing"}


## Future plans

- Implement consent management
- More privacy through private shards and cryptographic schemes
- Facilitate onboarding (prepaid guest accounts)
- Credentials Registry (upgrade NFT to VC by building client owned indexer)
- Decentralized storage (via Filecoin -> Machina)
- Cusdev and basic tokenomics

