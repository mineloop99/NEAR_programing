use crate::*;

/// external contract calls

#[ext_contract(ext_contract)]
trait ExtContract {
    fn nft_transfer(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
}
