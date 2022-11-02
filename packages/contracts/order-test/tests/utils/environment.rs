use super::builder::build_take_order_tx;
use super::builder::LimitOrder;
use fuel_core_interfaces::model::Coin;
use fuels::{
    contract::script::Script,
    prelude::{Provider, TxParameters},
    signers::{Signer, WalletUnlocked},
    test_helpers::{setup_single_asset_coins, setup_test_client, Config},
    tx::{Address, AssetId, Input, Output, Receipt, Transaction, TxPointer, UtxoId, Word},
};
pub async fn setup_environment(
    coin: (Word, AssetId),
) -> (WalletUnlocked, WalletUnlocked, Vec<Input>, Provider) {
    let mut wallet = WalletUnlocked::new_random(None);
    let mut wallet2 = WalletUnlocked::new_random(None);
    let mut all_coins: Vec<(UtxoId, Coin)> =
        setup_single_asset_coins(wallet.address(), coin.1, 1, coin.0);
    let mut coins2 = setup_single_asset_coins(wallet2.address(), coin.1, 1, coin.0 / 2);
    all_coins.append(&mut coins2);

    // Create the client and provider
    let mut provider_config = Config::local_node();
    provider_config.predicates = true;
    provider_config.utxo_validation = true;
    let (client, _) =
        setup_test_client(all_coins.clone(), Vec::new(), Some(provider_config), None).await;
    let provider = Provider::new(client);

    // Add provider to wallet
    wallet.set_provider(provider.clone());
    wallet2.set_provider(provider.clone());

    let coin_inputs: Vec<Input> = all_coins
        .into_iter()
        .map(|coin| Input::CoinSigned {
            utxo_id: UtxoId::from(coin.0.clone()),
            owner: Address::from(coin.1.owner.clone()),
            amount: coin.1.amount.clone().into(),
            asset_id: AssetId::from(coin.1.asset_id.clone()),
            tx_pointer: TxPointer::default(),
            witness_index: 0,
            maturity: 0,
        })
        .collect();
    (wallet, wallet2, coin_inputs, provider)
}

pub async fn take_order(
    order: LimitOrder,
    wallet: &WalletUnlocked,
    gas_coin: Input,
    predicate_coins_input: Input,
    optional_inputs: &[Input],
    optional_outputs: &[Output],
) -> Vec<Receipt> {
    let mut tx = build_take_order_tx(
        order,
        wallet.address().into(),
        gas_coin,
        predicate_coins_input,
        optional_inputs,
        optional_outputs,
        TxParameters::default(),
    )
    .await;

    sign_and_call_tx(wallet, &mut tx).await
}

pub async fn sign_and_call_tx(wallet: &WalletUnlocked, tx: &mut Transaction) -> Vec<Receipt> {
    let provider = wallet.get_provider().unwrap();
    wallet.sign_transaction(tx).await.unwrap();
    let script = Script::new(tx.clone());
    script.call(provider).await.unwrap()
}
