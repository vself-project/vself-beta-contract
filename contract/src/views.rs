use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use std::cmp::min;

use crate::*;

#[near_bindgen]
impl Contract {
    /// Returns semver of this contract.
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Event general status (on/off)
    pub fn is_active(&self) -> bool {
        match self.event {
            Some(_) => true,
            None => false
        }
    }

    /// Return current event data
    pub fn get_event_data(self) -> Option<EventData> {
        self.event
    }

    /// Return current event stats
    pub fn get_event_stats(self) -> Option<EventStats> {
        self.stats
    }

    /// Return user balance
    pub fn get_user_balance(&self, account_id: AccountId) -> Option<UserBalance> {
        self.balances.get(&account_id)
    }

    /// Get all user actions for current event (supports pagination)
    /// - `from_index` is the index to start from.
    /// - `limit` is the maximum number of elements to return.
    pub fn get_actions(&self, from_index: u64, limit: u64) -> Vec<ActionData> {  
        if let None = self.event { // No event is running -> no actions
            log!("No ongoing event, sorry.");
            return vec![];
        }
        let actions_from = self.actions_from.get(self.event_id).unwrap();
        let from_index = actions_from + from_index; // Shift for current event
        (from_index..std::cmp::min(from_index + limit, self.actions.len()))
            .map(|index| self.actions.get(index).unwrap())
            .collect()
    }

    /// Get all user actions for current event (supports pagination)
    /// - `from_index` is the index to start from.
    /// - `limit` is the maximum number of elements to return.
    pub fn get_past_event_actions(&self, event_id: u64, from_index: u64, limit: u64) -> Vec<ActionData> {
        assert!(self.event_id > event_id, "Wrong event index");
        let actions_from = self.actions_from.get(event_id).unwrap();
        let from_index = actions_from + from_index; // Shift for current event
        (from_index..std::cmp::min(from_index + limit, self.actions.len()))
            .map(|index| self.actions.get(index).unwrap())
            .collect()
    }

    /// Get past events data (supports pagination)
    /// - `from_index` is the index to start from.
    /// - `limit` is the maximum number of elements to return.
    pub fn get_past_events(&self, from_index: u64, limit: u64) -> Vec<(EventData, EventStats)> {          
        (from_index..std::cmp::min(from_index + limit, self.past_events.len()))
            .map(|index| self.past_events.get(index).unwrap())
            .collect()
    } 
}
