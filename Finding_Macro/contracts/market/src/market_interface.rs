use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::ValidAccountId;
use near_sdk::{near_bindgen, AccountId};
use serde::Serialize;
#[derive(BorshSerialize, BorshDeserialize, Serialize, Debug)]
pub enum ListingStatus {
    Active,
    Sold,
    Cancelled,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Debug)]
pub struct Listing {
    pub status: ListingStatus,
    pub seller: AccountId,
    pub token: AccountId,
    pub token_id: u128,
    pub price: u128,
}

#[near_bindgen]
impl Default for Listing {
    fn default() -> Self {
        Self {
            status: ListingStatus::Active,
            seller: String::from("aa"),
            token: String::from("aa"),
            token_id: 0,
            price: 0,
        }
    }
}

pub trait MarketInterface {
    fn list_nft(
        &mut self,
        nft_account_id: ValidAccountId,
        receiver_id: ValidAccountId,
        nft_id: u128,
        price: u128,
    );
    fn get_listing(&self, listing_id: u128) -> Option<Listing>;

    //fn get_amout_listing(&self, listing_id: u128) -> u128;

    //fn buy_nft(&mut self, listing_id: u128);
    //fn cancel(&mut self);
}
