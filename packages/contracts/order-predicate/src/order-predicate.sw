predicate;
use std::{b512::B512, constants::ZERO_B256, ecr::ec_recover_address, inputs::input_predicate_data};
use order::Order;
use std::hash::sha256;

// the constants that define each predicate. I would rather pass these as arguments, but i dont know how 
const ORDER_HASH: b256 = 0x356d49ebdbdffb1c16d761d7b8c51865aa2ab5a7a9b716b0b75e426a3082ac2e;
fn main() -> bool {
    let order: Order = input_predicate_data(0);
    // probably just pass the hash directly, but it is useful to back out this data 
    assert(sha256(order) == ORDER_HASH); // they passed some bs order 
    match order {
        Order::Limit(limit_order) => {
            let input_coin = input_coin_asset_id(0);
            let input_coin_amount = input_coin_amount(0);
            assert(input_coin == limit_order.taker_token);
            assert(input_coin_amount >= limit_order.taker_amount);
            // assert( <= order.exiry);
            true
        },
        _ => revert(0),
    }
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
