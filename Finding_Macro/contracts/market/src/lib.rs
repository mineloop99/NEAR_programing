mod market_interface;
mod nft;
mod nft_interface;
use market_interface::{Listing, ListingStatus, MarketInterface};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::json_types::ValidAccountId;
use near_sdk::serde_json::json;
use near_sdk::{env, near_bindgen, Balance, BorshStorageKey, CryptoHash, Gas, PanicOnDefault};
use near_sdk::{AccountId, Promise};
near_sdk::setup_alloc!();

const NO_DEPOSIT: Balance = 0;
const DEPOSIT: Balance = 1;
const BASIC_GAS: Gas = 5_000_000_000_000;
pub type FungibleTokenId = AccountId;
pub type ContractAndTokenId = String;
/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    ByListing,
    ByNFTContractId,
    ByNFTContractIdInner { account_id_hash: CryptoHash },
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Market {
    listing_id_counter: u128,
    listings: UnorderedMap<u128, Listing>,
    pub nft_contract_id: UnorderedSet<TokenId>,
}

#[near_bindgen]
impl Market {
    #[init]
    fn constructor() -> Self {
        Self {
            listing_id_counter: 0,
            listings: UnorderedMap::new(StorageKey::ByListing),
            nft_contract_id: UnorderedSet::new(StorageKey::ByNFTContractId),
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
        let account_id: &AccountId = &nft_account_id.into();
        let receiver_id: AccountId = receiver_id.into();
        Promise::new(account_id.to_string()).function_call(
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
        let listing_new = &Listing {
            status: ListingStatus::Active,
            seller: receiver_id.into(),
            token: account_id.clone(),
            token_id: nft_id,
            price: price,
        };
        self.listings.insert(&self.listing_id_counter, listing_new);
        self.listing_id_counter += 1;
        env::log(b"NFT Listing Call");
    }

    fn get_listing(&self, listing_id: u128) -> Option<Listing> {
        self.listings.get(&listing_id)
    }

    fn nft_fractionalize(&mut self, ) {

    }
    // fn buy_nft(&mut self, listing_id: u128) {}

    // fn get_amout_listing(&self, listing_id: u128) -> u128 {
    //     listing_id
    // }

    // fn cancel(&mut self) {}
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use crate::nft::Nft;
    use crate::nft_interface::NftInterface;

    use super::*;
    use near_contract_standards::non_fungible_token::approval::NonFungibleTokenApproval;
    use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;
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
        assert_eq!(contract.get_listing(0).is_none(), true);
    }

    #[test]
    fn test_mint_and_listing() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Market::constructor();
        let mut nft_contract = Nft::new_default_meta(accounts(0));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let nft_meta = &near_contract_standards::non_fungible_token::metadata::TokenMetadata {
            reference: None,
            reference_hash: None,
            title: None,
            description: None,
            media: None,
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
        };
        nft_contract.nft_mint(String::from("0"), accounts(0), nft_meta.clone());
        nft_contract.nft_approve(String::from("0"), accounts(1), None);
        contract.list_nft(accounts(0), accounts(1), 0, 10);
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        nft_contract.nft_transfer(accounts(1), String::from("0"), None, None);
    }
}
