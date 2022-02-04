mod constants;

use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
  };
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde_json::json;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, PanicOnDefault, AccountId, BorshStorageKey, Promise, PromiseResult, PromiseOrValue
};
use near_sdk::collections::{ UnorderedMap, LazyOption};

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct QuestData {
    pub qr_prefix: String,        
    pub reward: TokenId,
}

// Current event data
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct EventData {            
    event_active: bool,
    event_name: String,
    event_description: String,
    start_time: u64,
    finish_time: u64,
    quests: Vec<QuestData>,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CheckInData {
    username: String,
    timestamp: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    event_id: u64, // Current event count

    // NFT implementation
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,

    // Event statistics and metadata
    event: LazyOption<EventData>,
    users: Vector<AccountId>, // Participants
    stats: UonorderedMap<AccountId, UserStats>, // User stats    

    // Past events
    past_events: LookupMap<u64, EventData>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Checkins,
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

// Contract NFT metadata
use constants::DATA_IMAGE_SVG_NEAR_ICON;
use constants::BASE_URI;

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).    
    #[init]
    pub fn new() -> Self {        
        assert!(!env::state_exists(), "Already initialized");
        let owner_id = env::current_account_id(); // Who deployed owns

        let metadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "vSelf NEAR NFT checkins".to_string(),
            symbol: "VSLF".to_string(),
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            base_uri: Some(BASE_URI.to_string()),
            reference: None,
            reference_hash: None,
        };
        metadata.assert_valid();
        
        let test_event_data = EventData { // TO DO move to constants: mock_event_data();
            event_active: false,
            event_name: "Test Event".to_string(),
            event_description: "Test Event Description".to_string(),            
            start_time: 0,
            finish_time: 0,
            quests: vec![ 
                QuestData{ qr_prefix: "http://".to_string(), reward: "HI".to_string() }, 
                QuestData{ qr_prefix: "https://".to_string(), reward: "HI".to_string() }
                ],
        };

        Self {
            owner_id: owner_id.clone().into(),
            event_data: test_event_data,            
            
            checkins: UnorderedMap::new(StorageKey::Checkins),
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),            
        }                
    }
      
    pub fn toss(&mut self) -> u8 {        
        // Toss the dice (minimal logic for now)
        let rand: u8 = *env::random_seed().get(0).unwrap();
        
        return rand;
    }
    
    pub fn start(&mut self) {
        self.event_data.event_active = true;
    }

    pub fn abort(&mut self) {        
        self.event_data.event_active = false;
    }

    #[payable]
    pub fn checkin(&mut self, username: String, request: String) -> u8 {
        // Assert event is active
        assert!( self.event_data.event_active );

        // Register checkin data
        let timestamp: u64 = env::block_timestamp();

        let checkin_data = CheckInData {
            username: username.clone(),
            timestamp: timestamp,
        };

        let mut checkins = self.checkins.get(&request).unwrap_or(vec![]);
        checkins.push(checkin_data);
        self.checkins.insert(&request, &checkins);
        
        // Decide what to transfer for the player
        return 0;
    }

    /// Mint a new token with ID=`token_id` belonging to `receiver_id`.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. `self.tokens.mint` will also require it to be Some, since
    /// `StorageKey::TokenMetadata` was provided at initialization.
    ///
    /// `self.tokens.mint` will enforce `predecessor_account_id` to equal the `owner_id` given in
    /// initialization call to `new`.
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.tokens.internal_mint(token_id, receiver_id, Some(token_metadata))
    }

    /// Views 

    // Event general status
    pub fn is_active(&self) -> bool {
        self.event_data.event_active
    }

    // Get event metadata
    pub fn get_event_metadata() {

    }

    // Get records for specific QR request
    pub fn get_event_stats(&self, event_id: u64) -> Vec<CheckInData> {  
        self.checkins.get(&request).unwrap_or(vec![])
    }
}

// Implement NFT standart
near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
  fn nft_metadata(&self) -> NFTContractMetadata {
      self.metadata.get().unwrap()
  }
}

// Tests TO DO