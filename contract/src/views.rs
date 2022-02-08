use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use std::cmp::min;

use crate::*;

#[near_bindgen]
impl Contract {
    /// Returns semver of this contract.
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    // Event general status
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
    // pub fn get_user_rewards_balance() -> Vec<bool> {

    // }

    // Get all user actions for current event
    /// - `from_index` is the index to start from.
    /// - `limit` is the maximum number of elements to return.
    pub fn get_event_actions(&self, from_index: u64, limit: u64) -> Vec<ActionData> {  
        let from_index = self.actions_from + from_index; // Shift for current event
        (from_index..std::cmp::min(from_index + limit, self.actions.len()))
            .map(|index| self.actions.get(index).unwrap())
            .collect()
    }
}
