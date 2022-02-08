use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
  };
use near_contract_standards::non_fungible_token::{Token, TokenId, NonFungibleToken};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde_json::json;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, Gas, Balance, PanicOnDefault, AccountId, BorshStorageKey, Promise, PromiseResult, PromiseOrValue
};
use near_sdk::collections::{ LookupMap, LazyOption, Vector };
use std::collections::HashSet;

mod constants;
pub mod views;
pub mod nft;

use near_sdk::ONE_YOCTO;
use constants::SINGLE_CALL_GAS;

#[derive(Clone)]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct QuestData {
    pub qr_prefix: String,    
    pub reward_title: String,
    pub reward_description: String,
    pub reward_url: String,
}

// Current event data
#[derive(Clone)]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventData {                
    event_name: String,
    event_description: String,
    start_time: u64,
    finish_time: u64,
    quests: Vec<QuestData>,
}

// Current event data
#[derive(Clone)]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventStats {               
    participants: HashSet<AccountId>, // Participants of current event
    start_time: u64,
    finish_time: Option<u64>,
    total_rewards: u64,
    total_users: u64,
}

#[derive(Clone)]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ActionData {
    timestamp: u64,
    username: String,
    qr_string: String,
    reward_index: usize,    
}

/// This is format of output via JSON for the user balance.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserBalance {
    pub karma_balance: u64,
    pub quests_status: Vec<bool>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId, // Owner ID
    event_id: u64, // Current event count
    event: Option<EventData>, // Current event metadata
    stats: Option<EventStats>, // Event stats aggregated for current event

    // NFT implementation
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,

    // Event statistics and history   
    last_action_index: u64, // Last action index
    actions_from: u64, // Current event actions start from that index
    actions: Vector<ActionData>, // History of all user actions
    
    /// Balance sheet for each user
    balances: LookupMap<AccountId, UserBalance>,

    /// Past events archive
    past_events: Vector<(EventData, EventStats)>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Actions,
    Balances,
    PastEvents,
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

        Self {
            owner_id: owner_id.clone().into(),
            event_id: 0,
            event: None,
            stats: None,
            last_action_index: 0,
            actions_from: 0,
            actions: Vector::new(StorageKey::Actions),
            balances: LookupMap::new(StorageKey::Balances),
            past_events: Vector::new(StorageKey::PastEvents),
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
    
    // Initiate next event
    pub fn start_event(&mut self) {
        assert!( self.event.is_none() );
        let timestamp: u64 = env::block_timestamp();        

        let test_data = constants::mock_event_data();
        self.event = Some(test_data);
        self.stats = Some(EventStats {
            participants: HashSet::new(),
            start_time: timestamp,
            finish_time: None,
            total_rewards: 0,
            total_users: 0,
        })
    }

    // Stop and put event to archive
    pub fn stop_event(&mut self) {
        assert!( self.event.is_some() );
        let timestamp: u64 = env::block_timestamp();        

        let mut final_stats = self.stats.as_ref().unwrap().clone(); 
        final_stats.finish_time = Some(timestamp);

        let mut final_event_data = self.event.as_ref().unwrap().clone();

        self.past_events.push(&(final_event_data, final_stats));
        self.event_id += 1;
        self.actions_from = self.last_action_index;

        self.event = None;
        self.stats = None;
    }

    #[payable]
    pub fn checkin(&mut self, username: String, request: String) -> usize {
        // Assert event is active
        assert!( self.event.is_some() );        

        let timestamp: u64 = env::block_timestamp();
        let qr_string = request.clone();
        
        // Match QR code to quest
        let quests = self.event.as_ref().unwrap().quests.clone();        
        let mut reward_index = 0;
        for quest in &quests {            
            if request.starts_with(&quest.qr_prefix) { break };
            reward_index = reward_index + 1;
        }

        let quest = quests.get(reward_index).unwrap();
        let action_data = ActionData {
            username: username.clone(),
            qr_string: qr_string.clone(),
            reward_index,
            timestamp,
        };

        // Register checkin data
        self.last_action_index += 1;
        self.actions.push(&action_data);
        // TO DO
        // if let Some(stats) = self.stats {
        //     if stats.users.insert(username);
        //     self.stats = EventStats {
        //         users: 
        //         total_rewards
        //     }           
        // }
        log!("Checkin successful! User: {}, Quest: {}", username, reward_index.clone());                

        // Check if account seems valid
        if !AccountId::try_from(username.clone()).is_ok() {
            return reward_index;
        }
        let token_id_with_timestamp: String = format!("{}:{}:{}", reward_index.clone(), timestamp); 
        let contract_id = env::current_account_id();        
        let root_id = AccountId::try_from(contract_id).unwrap();
        
        // Decide what to transfer for the player                                                                
        let media_url: String = format!("{}", quest.reward_url);
        let media_hash = Base64VecU8(env::sha256(media_url.as_bytes()));
        
        let mut token_metadata = TokenMetadata {
            title: Some(quest.reward_title.clone()),
            description: Some(quest.reward_description.clone()),
            media: Some(media_url),
            media_hash: Some(media_hash),
            copies: Some(1u64),
            issued_at: Some(timestamp.to_string()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: Some(qr_string.clone()),
            reference: None,
            reference_hash: None,
        };

        // Mint achievement reward                
        self.nft_mint(token_id_with_timestamp.clone(), root_id.clone(), token_metadata.clone());
        log!("Success! Minting NFT for {}! TokenID = {}", root_id.clone(), token_id_with_timestamp.clone());
        
        // Transfer NFT to new owner
        let receiver_id = AccountId::try_from(username).unwrap();                
        env::promise_create(
            root_id.clone(),
            "nft_transfer",
            json!({
                "token_id": token_id_with_timestamp,
                "receiver_id": receiver_id,
            })
            .to_string()
            .as_bytes(),
            ONE_YOCTO,
            SINGLE_CALL_GAS,
        );
        log!("Success! Transfering NFT for {} from {}", receiver_id.clone(), root_id.clone());
        
        return reward_index;
    }    
}

// Tests TO DO