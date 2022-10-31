predicate;
use std::{b512::B512, constants::ZERO_B256, ecr::ec_recover_address, inputs::input_predicate_data};
use order::*;

// update this with the script for spending
const SPENDING_SCRIPT_HASH = 0x7994ce111c78a2539cedce7ad0476b01089c6d7060095172e3f6df5a3564d949;
// the constants that define each predicate. I would rather pass these as arguments, but i dont know how 
// cant pass the hash because that causes circular dependency 
fn main(take_coin: b256, min_take_amount: u64) -> bool {
    let take_coin: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let min_take_amount: u64 = 10;
    // parameterize this thing
    // let order: LimitOrder = input_predicate_data(0);
    // probably just pass the hash directly, but it is useful to back out this data 
    // assert(sha256(order) == ORDER_HASH); // they passed some bs order 
    let input_coin = input_coin_asset_id(0);
    let input_coin_amount = input_coin_amount(0);
    assert(tx_script_bytecode_hash() == SPENDING_SCRIPT_HASH);
    assert(input_coin == take_coin);
    assert(input_coin_amount >= min_take_amount);
    // need to assert that there is an actually transfer 
    let output_coin = output_coin_asset_id();
    true
}

const GTF_INPUT_COIN_AMOUNT = 0x105;
const GTF_INPUT_COIN_ASSET_ID = 0x106;
const GTF_SCRIPT_SCRIPT_LENGTH = 0x005;
const GTF_SCRIPT_SCRIPT = 0x00B;
pub fn input_coin_asset_id(index: u64) -> b256 {
    __gtf::<b256>(index, GTF_INPUT_COIN_ASSET_ID)
}

/// Get the amount of a coin input
pub fn input_coin_amount(index: u64) -> u64 {
    __gtf::<u64>(index, GTF_INPUT_COIN_AMOUNT)
}

/// Get the hash of the script bytecode
pub fn tx_script_bytecode_hash() -> b256 {
    let mut result_buffer = ZERO_B256;
    asm(hash: result_buffer, ptr: __gtf::<u64>(0, GTF_SCRIPT_SCRIPT), len: __gtf::<u64>(0, GTF_SCRIPT_SCRIPT_LENGTH)) {
        s256 hash ptr len;
        hash: b256
    }
}


////////////
// OUTPUT //
////////////
// /// Get the transaction outputs count
// pub fn output_count() -> u64 {
//     __gtf::<u64>(0, GTF_SCRIPT_OUTPUTS_COUNT)
// }

fn verify_output_coin() -> b256 {
    __gtf::<u64>(index, GTF_OUTPUT_TYPE) == OUTPUT_TYPE_COIN
}
