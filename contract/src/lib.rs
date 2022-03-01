use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NFT_METADATA_SPEC,
  };
use near_contract_standards::non_fungible_token::{NonFungibleToken};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde_json::json;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, PanicOnDefault, AccountId, BorshStorageKey};
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
    pub reward_uri: String,
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
    total_actions: u64,
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

#[derive(Clone)]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ActionResult {
    index: usize,
    got: bool,
    title: String,
    description: String,
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
    actions_from: Vector<u64>, // Event actions start from that index
    actions: Vector<ActionData>, // History of all user actions
    
    /// Balance sheet for each user
    balances: LookupMap<AccountId, UserBalance>,

    /// Past events archive
    past_events: Vector<(EventData, EventStats)>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Actions,
    ActionsFrom,
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
            name: "vSelf Metabuild Event Quest".to_string(),
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
            actions_from: Vector::new(StorageKey::ActionsFrom),
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
    pub fn start_event(&mut self, event: EventData) {
        assert!( self.event.is_none() );        
        let timestamp: u64 = env::block_timestamp();        

        self.actions_from.push(&self.last_action_index);

        self.event = Some(event);
        self.stats = Some(EventStats {
            participants: HashSet::new(),
            start_time: timestamp,
            finish_time: None,
            total_rewards: 0,
            total_users: 0,
            total_actions: 0,
        })
    }

    // Stop and put event to archive
    pub fn stop_event(&mut self) {
        assert!( self.event.is_some() );
        let timestamp: u64 = env::block_timestamp();        

        let mut final_stats = self.stats.as_ref().unwrap().clone(); 
        final_stats.finish_time = Some(timestamp);
        let final_event_data = self.event.as_ref().unwrap().clone();        

        self.past_events.push(&(final_event_data, final_stats));
        self.event_id += 1;        

        self.event = None;
        self.stats = None;
    }

    #[payable]
    pub fn checkin(&mut self, username: String, request: String) -> Option<ActionResult> {
        // Assert event is active
        assert!( self.event.is_some(), "No event is running" );
        let timestamp: u64 = env::block_timestamp();        

        // Check if account seems valid
        assert!( AccountId::try_from(username.clone()).is_ok(), "Valid account is required" );
        let user_account_id = AccountId::try_from(username.clone()).unwrap();
                        
        // Match QR code to quest
        let qr_string = request.clone();
        let quests = self.event.as_ref().unwrap().quests.clone();        
        let mut reward_index = 0;
        for quest in &quests {            
            if request.starts_with(&quest.qr_prefix) { break };
            reward_index = reward_index + 1;
        }
        
        let action_data = ActionData {
            username: username.clone(),
            qr_string: qr_string.clone(),
            reward_index,
            timestamp,
        };        

        // Register checkin data        
        let mut stats = self.stats.as_ref().unwrap().clone();

        // Check if we have a new user
        if stats.participants.insert(user_account_id.clone()) {
            stats.total_users += 1;            
            
            self.balances.insert(&user_account_id, &UserBalance {
                karma_balance: 0,
                quests_status: vec![false; quests.len()],
            });
        }

        // Register action        
        self.last_action_index += 1;
        self.actions.push(&action_data.clone());
        stats.total_actions += 1;

        // Check if we've been awarded a reward
        if let Some(quest) = quests.get(reward_index) {  
            // Update state if we are lucky          
            stats.total_rewards += 1;          
            self.stats = Some(stats);

            // Update user balance
            let mut balance = self.balances.get(&user_account_id).expect("ERR_NOT_REGISTERED");
            balance.karma_balance += 1; // Number of successfull actions

            // Do we have this reward already            
            if (balance.quests_status[reward_index]) { // Yes
                self.balances.insert(&user_account_id, &balance);
                return Some(ActionResult {
                    index: reward_index,
                    got: true,
                    title: quest.reward_title.clone(),
                    description: quest.reward_description.clone(),
                });                
            } else { // No
                balance.quests_status[reward_index] = true;
                self.balances.insert(&user_account_id, &balance);

                // NFT Part (issue token)
                self.issue_nft_reward(user_account_id.clone(), reward_index.clone());                  

                return Some(ActionResult {
                    index: reward_index,
                    got: false,
                    title: quest.reward_title.clone(),
                    description: quest.reward_description.clone(),
                });
            }                                     
        } else {
            // Update state if we are not
            self.stats = Some(stats);       
            log!("No reward for this checkin! User: {}", username);
            return None;            
        }                        
    }
}

// Tests TO DO