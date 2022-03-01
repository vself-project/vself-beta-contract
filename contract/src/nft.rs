use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata,
  };
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::{
    env, log, near_bindgen, AccountId, Promise, PromiseOrValue
};

use crate::*;

#[near_bindgen]
impl Contract {
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

    /// Issue reward token
    #[payable]
    pub fn issue_nft_reward(&mut self, receiver_id: AccountId, reward_index: usize) {
        // Decide what to transfer for the player
        let contract_id = env::current_account_id();
        let timestamp: u64 = env::block_timestamp();

        let quest = self.event.as_ref().unwrap().quests.get(reward_index).unwrap();                                                                     
        let token_id_with_timestamp: String = format!("{}:{}:{}", self.event_id.clone(), reward_index.clone(), timestamp);
        let media_url: String = format!("{}", quest.reward_uri);
        let media_hash = Base64VecU8(env::sha256(media_url.as_bytes()));

        let token_metadata = TokenMetadata {
            title: Some(quest.reward_title.clone()),
            description: Some(quest.reward_description.clone()),
            media: Some(media_url),
            media_hash: Some(media_hash),
            copies: Some(1u64),
            issued_at: Some(timestamp.to_string()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };

        // Mint achievement reward                                
        let root_id = AccountId::try_from(contract_id).unwrap();

        self.nft_mint(token_id_with_timestamp.clone(), root_id.clone(), token_metadata.clone());
        log!("Success! Minting NFT for {}! TokenID = {}", root_id.clone(), token_id_with_timestamp.clone());

        // Transfer NFT to new owner
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