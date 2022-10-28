predicate;
use std::{b512::B512, constants::ZERO_B256, ecr::ec_recover_address, inputs::input_predicate_data};


// figure out how to dynamically construct a predicate, 
// there is this concept of predicate inputs in the SDK 

fn main() -> bool {
    let order: LimitOrder = input_predicate_data(0);
    let input_coin: b256 = input_coin_asset_id(0);
    let input_coin_amount: u64= input_coin_asset_id(0);
    assert(input_coin == order.taker_token);
    assert(input_count_amount >= oder.taker_amount);
    assert( <= order.exiry);
}
