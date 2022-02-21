use near_sdk::Gas;
use crate::*;

// Prepaid gas for making a single simple call.
pub const SINGLE_CALL_GAS: Gas = Gas(200000000000000);
pub const BASE_URI: &str = "https://vself-dev.web.app"; // TO DO IPFS

pub fn mock_event_data() -> EventData {
    EventData {
        event_name: "vSelf Onboarding Metabuild Quest".to_string(),
        event_description: "vSelf lauches a series of quests which will keep you motivated while you learn about our project and its place inside NEAR ecosystem".to_string(),            
        start_time: 0,
        finish_time: 0,
        quests: vec![ 
            QuestData{ 
                qr_prefix: "https://vself-dev.web.app/vself.apk".to_string(),                
                reward_title: "vSelf: Welcome Badge".to_string(),
                reward_description: "Welcome to the vSelf demo!".to_string(),
                reward_url: "https://vself-dev.web.app/nft1.png".to_string(),
            }, 
            QuestData{
                qr_prefix: "You have registered in the NEAR community".to_string(),
                reward_title: "vSelf: NEAR User Badge".to_string(),
                reward_description: "You have registered in the NEAR community".to_string(),
                reward_url: "https://vself-dev.web.app/nft2.png".to_string(),
            },
            QuestData{ 
                qr_prefix: "Congrats! Now you know more about Web3".to_string(),                
                reward_title: "vSelf: Early Adopter Badge".to_string(),
                reward_description: "Congrats! Now you know more about Web3".to_string(),
                reward_url: "https://vself-dev.web.app/nft3.png".to_string(),
            },
            QuestData{ 
                qr_prefix: "Thank you <3 and see you soon!".to_string(),                
                reward_title: "vSelf: Love Badge".to_string(),
                reward_description: "Thank you <3 and see you soon!".to_string(),
                reward_url: "https://vself-dev.web.app/nft4.png".to_string(),
            }          
        ],
    }
}

// Converted with https://yoksel.github.io/url-encoder/
pub const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";