predicate;
use std::{b512::B512, constants::ZERO_B256, ecr::ec_recover_address, inputs::input_predicate_data};


// figure out how to dynamically construct a predicate, 
// there is this concept of predicate inputs in the SDK 

// note that predicates are not deployed to the chain. rather, they are just a hash behind which some coins are locked.
// if you can provide the bytecode that generates that hash AND have it evaluate to true, then ta-da you can spend it

// This means that when creating an order, you just have to send coins to the code hash

// when executing the order, you just need the tx to make the predicate return true. 

// Truly exciting stuff
fn main() -> bool {
    let order: Order = input_predicate_data(0);
    match order {
        Order::Limit => {
            let input_coin: b256 = input_coin_asset_id(0);
            let input_coin_amount: u64= input_coin_asset_id(0);
            assert(input_coin == order.taker_token);
            assert(input_count_amount >= oder.taker_amount);
            // assert( <= order.exiry);
        },
        _ => revert(0)
    }

}
