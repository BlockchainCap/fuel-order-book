use fuels::{
    prelude::TxParameters,
    tx::{Input, Output, Transaction, Bytes32, AssetId, Address, Word},
};

const MIN_GAS: u64 = 100_000;
const MAKE_ORDER_SCRIPT_BINARY: &str = "../order-script/out/debug/order-script.bin";
pub async fn get_make_order_script() -> Vec<u8> {
    let script_bytecode = std::fs::read(MAKE_ORDER_SCRIPT_BINARY).unwrap();
    script_bytecode
}
const TAKE_ORDER_SCRIPT_BINARY: &str = "../order-script/out/debug/make-order.bin";
pub async fn get_take_order_script() -> Vec<u8> {
    let script_bytecode = std::fs::read(TAKE_ORDER_SCRIPT_BINARY).unwrap();
    script_bytecode
}

pub async fn build_make_order_tx(
    gas_coin: Input,
    optional_inputs: &[Input],
    optional_outputs: &[Output],
    params: TxParameters,
) -> Transaction {
    let script_bytecode = get_make_order_script().await;
    let mut tx_outputs: Vec<Output> = Vec::new();
    
    // TODO: probably dont need this because we arent calling a contract
    tx_outputs.push(Output::Contract {
        input_index: 0u8,
        balance_root: Bytes32::zeroed(),
        state_root: Bytes32::zeroed(),
    });

    // Build a change output for the owner of the provided gas coin input
    match gas_coin {
        Input::CoinSigned { owner, .. } | Input::CoinPredicate { owner, .. } => {
            // Add change output
            tx_outputs.push(Output::Change {
                to: owner.clone(),
                amount: 0,
                asset_id: AssetId::default(),
            });
        }
        _ => {
            // do nothing
        }
    }

    // Build variable output
    tx_outputs.push(Output::Variable {
        to: Address::default(),
        amount: Word::default(),
        asset_id: AssetId::default(),
    });

    // Start building tx list of inputs
    let mut tx_inputs: Vec<Input> = Vec::new();
    tx_inputs.push(gas_coin);

    // Append provided inputs and outputs
    tx_inputs.append(&mut optional_inputs.to_vec());
    tx_outputs.append(&mut optional_outputs.to_vec());

    // Create the trnsaction
    Transaction::Script {
        gas_price: params.gas_price,
        gas_limit: MIN_GAS,
        maturity: params.maturity,
        receipts_root: Default::default(),
        script: script_bytecode,
        script_data: vec![],
        inputs: tx_inputs,
        outputs: tx_outputs,
        witnesses: vec![],
        metadata: None,
    }
}

pub async fn build_take_order_tx(
    gas_coin: Input,
    optional_inputs: &[Input],
    optional_outputs: &[Output],
    params: TxParameters,
) -> Transaction {
    let script_bytecode = get_take_order_script().await;
    todo!();
}
