use near_contract_standards::non_fungible_token::{
    metadata::{NFTContractMetadata, TokenMetadata},
    Token, TokenId,
};
use near_sdk::{json_types::ValidAccountId, Promise};

pub trait NftInterface {
    fn new_default_meta(owner_id: ValidAccountId) -> Self;

    fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata) -> Self;

    fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: ValidAccountId,
        token_metadata: TokenMetadata,
    ) -> Token;
}
