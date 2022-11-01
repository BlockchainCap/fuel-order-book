predicate;
use std::{
    b512::B512,
    constants::ZERO_B256,
    ecr::ec_recover_address,
    inputs::input_predicate_data,
    revert::require,
};
use order::*;

// update this with the script for spending
const SPENDING_SCRIPT_HASH = 0xf7b2f6690ccb273d65fa54e6e02be86c03d6e55c21367fc855e9366720c317f8;
// the constants that define each predicate. I would rather pass these as arguments, but i dont know how 
const OUTPUT_COIN_INDEX = 0u8;
// cant pass the hash because that causes circular dependency 
fn main(take_coin: b256, min_take_amount: u64) -> bool {
    let take_coin: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let min_take_amount: u64 = 10;
    let maker: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    // parameterize this thing
    // let order: LimitOrder = input_predicate_data(0);
    // probably just pass the hash directly, but it is useful to back out this data 
    // assert(sha256(order) == ORDER_HASH); // they passed some bs order 
    let input_coin = input_coin_asset_id(0);
    let input_coin_amount = input_coin_amount(0);
    assert(tx_script_bytecode_hash() == SPENDING_SCRIPT_HASH);
    assert(input_coin == take_coin); // this 
    assert(input_coin_amount >= min_take_amount);

    assert(output_count() == 1);
    assert(verify_output_coin(OUTPUT_COIN_INDEX));
    assert(output_coin_asset_id(OUTPUT_COIN_INDEX) == take_coin);
    assert(output_coin_amount(OUTPUT_COIN_INDEX) >= min_take_amount);
    
    // this is the one that is failing
    assert(output_coin_to(OUTPUT_COIN_INDEX) == maker);
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
/// Get the transaction outputs count
const GTF_SCRIPT_OUTPUTS_COUNT = 0x008;
const GTF_OUTPUT_TYPE = 0x201;
const OUTPUT_TYPE_COIN = 0u8; // again... not sure aboue this type here. 
const GTF_OUTPUT_COIN_TO: u64 = 0x202;
const GTF_OUTPUT_COIN_AMOUNT: u64 = 0x203;
const GTF_OUTPUT_COIN_ASSET_ID: u64 = 0x204;
pub fn output_count() -> u64 {
    __gtf::<u64>(0, GTF_SCRIPT_OUTPUTS_COUNT)
}
fn verify_output_coin(index: u64) -> bool {
    __gtf::<u64>(index, GTF_OUTPUT_TYPE) == OUTPUT_TYPE_COIN
}

fn output_coin_asset_id(index: u64) -> b256 {
    __gtf::<b256>(index, GTF_INPUT_COIN_ASSET_ID)
}
fn output_coin_amount(index: u64) -> u64 {
    __gtf::<u64>(index, GTF_OUTPUT_COIN_AMOUNT)
}
fn output_coin_to(index: u64) -> b256 {
    __gtf::<b256>(index, GTF_OUTPUT_COIN_TO)
}
