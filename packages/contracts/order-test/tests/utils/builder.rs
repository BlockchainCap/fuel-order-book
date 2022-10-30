use std::process::Output;

use fuels::{tx::{Transaction, Input}, prelude::TxParameters};

/// Gets the message to contract script
pub async fn get_make_order_script() -> Vec<u8> {
    let script_bytecode = std::fs::read(CONTRACT_MESSAGE_SCRIPT_BINARY).unwrap();
    script_bytecode
}


pub async fn create_make_order_tx(
    gas_coin: Input,
    optional_inputs: &[Input],
    optional_outputs: &[Output],
    params: TxParameters,
) -> Transaction {
    let script_bytecode = get_make_order_script().await;
    
}

// pub async fn create_take_order_tx() -> Transaction {

// }
