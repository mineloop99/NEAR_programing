mod market_interface;
use market_interface::{Listing, ListingStatus, MarketInterface};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::ValidAccountId;
use near_sdk::serde_json::json;
use near_sdk::{env, log, near_bindgen, Balance, BorshStorageKey, Gas, PanicOnDefault};
use near_sdk::{AccountId, Promise};
near_sdk::setup_alloc!();

const NO_DEPOSIT: Balance = 0;
const DEPOSIT: Balance = 1;
const BASIC_GAS: Gas = 5_000_000_000_000;

#[near_bindgen]
#[derive(BorshStorageKey, BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Market {
    listing_id_counter: u128,
    listings: Option<UnorderedMap<u128, Listing>>,
}
#[near_bindgen]
impl Market {
    #[init]
    fn constructor() -> Self {
        Self {
            listing_id_counter: 0,
            listings: None,
        }
    }
}
#[near_bindgen]
impl MarketInterface for Market {
    #[payable]
    fn list_nft(
        &mut self,
        nft_account_id: ValidAccountId,
        receiver_id: ValidAccountId,
        nft_id: u128,
        price: u128,
    ) {
        let account_id: AccountId = nft_account_id.into();
        let receiver_id: AccountId = receiver_id.into();
        Promise::new(account_id).function_call(
            b"nft_transfer_call".to_vec(),
            json!({
            "receiver_id": receiver_id,
            "token_id":nft_id.to_string(),
            "approval_id": 0,
            "msg":"hello"
            })
            .to_string()
            .as_bytes()
            .to_vec(),
            DEPOSIT,
            BASIC_GAS,
        );
    }

    fn get_listing(&self, listing_id: u128) -> Listing {
        let a = &self.listings;
        match a {
            None => Listing::default(),
            Some(value) => match value.get(&listing_id) {
                None => Listing::default(),
                Some(listing) => listing,
            },
        }
    }

    fn fractionalize(&mut self,)

    // fn buy_nft(&mut self, listing_id: u128) {}

     
    // fn get_amout_listing(&self, listing_id: u128) -> u128 {
    //     listing_id
    // }

    // fn cancel(&mut self) {}
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, PromiseResult};
    const MINT_STORAGE_COST: Balance = 5870000000000000000000;
    const BASIC_GAS: Gas = 5_000_000_000_000;
    const CODE: &[u8] = include_bytes!("../../../out/nft.wasm");

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_get_listing() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Market::constructor();
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.get_listing(0).token_id, 0);
    }

    #[test]
    fn test_mint_and_listing() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let token_id = 0;
        let owner_id: AccountId = accounts(0).into();
        let receiver_id: AccountId = accounts(1).into();
        let nft_contract = proj1::nft_interface::NftInterface::new_default_meta();
        let mut contract = Market::constructor();
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build()); 
        contract.list_nft(accounts(0), accounts(1), 0, 10);
    }
}
