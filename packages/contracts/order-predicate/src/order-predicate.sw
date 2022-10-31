predicate;
use std::{b512::B512, constants::ZERO_B256, ecr::ec_recover_address, inputs::input_predicate_data};
use order::*;
use std::hash::sha256;

// the constants that define each predicate. I would rather pass these as arguments, but i dont know how 
// cant pass the hash because that causes circular dependency 
// const ORDER_HASH: b256 = 0x356d49ebdbdffb1c16d761d7b8c51865aa2ab5a7a9b716b0b75e426a3082ac2e;
fn main() -> bool {
    // parameterize these thing
    let maker_coin = "";
    let maker_amount = 1;
    let taker_coin = "";

    let order: LimitOrder = input_predicate_data(0);
    // probably just pass the hash directly, but it is useful to back out this data 
    // assert(sha256(order) == ORDER_HASH); // they passed some bs order 
    let input_coin = input_coin_asset_id(0);
    let input_coin_amount = input_coin_amount(0);
    assert(input_coin == order.taker_token);
    assert(input_coin_amount >= order.taker_amount);
    // assert( <= order.exiry);
    true
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
