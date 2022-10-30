script;
use std::token::transfer_to_address;
use std::contract_id::ContractId;

// the create order script just send coins to a predicate
// need to calculate the predicate hash tho
const predicate_root = ~Address::from(0x356d49ebdbdffb1c16d761d7b8c51865aa2ab5a7a9b716b0b75e426a3082ac2e);

fn main() {
    let amount = input_coin_amount(0);
    let asset_id = input_coin_asset_id(0);
    // transfer coins to the predicate root
    transfer_to_address(amount, ~ContractId::from(asset_id), predicate_root);
}

const GTF_INPUT_COIN_AMOUNT = 0x105;
const GTF_INPUT_COIN_ASSET_ID = 0x106;
pub fn input_coin_asset_id(index: u64) -> b256 {
    __gtf::<b256>(index, GTF_INPUT_COIN_ASSET_ID)
}

/// Get the amount of a coin input
pub fn input_coin_amount(index: u64) -> u64 {
    __gtf::<u64>(index, GTF_INPUT_COIN_AMOUNT)
}
