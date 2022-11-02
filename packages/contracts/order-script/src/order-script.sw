script;
use std::token::transfer_to_address;
use std::contract_id::ContractId;

// the create order script just send coins to a predicate
// need to calculate the predicate hash tho
const predicate_root = ~Address::from(0x55af381775fabd6de7f25013a794ffc02e60ab948deeac3bc4a9db27003e9084);

// to take an order need to send funds to the correct receiver address and then spend the predicate

fn main() {
    let amount = input_coin_amount(0);
    let asset_id = input_coin_asset_id(0);
    // this thing needs to transfer some coins then spend the predicate 
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
