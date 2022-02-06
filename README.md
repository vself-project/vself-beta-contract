## vSelf Beta Event Management Contract

### Deployment

1. testnet => event.vself.testnet
1. mainnet => TBD

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

### Contract Interface

Change Methods:

1. start_event()
1. finish_event()
1. checkin()

Read-only Methods:

1. get_event_data()
1. get_event_actions()
1. get_event_stats()

## Future plans

- Implement consent management
- Facilitate onboarding (prepaid guest accounts)
- Credentials Registry (upgrade NFT to VC by building client owned indexer)
- Decentralized storage (via Filecoin -> Machina)
- Utility token and basic tokenomics

