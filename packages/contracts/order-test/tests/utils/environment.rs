use fuel_core_interfaces::model::Coin;
use fuels::{
    prelude::Provider,
    signers::WalletUnlocked,
    test_helpers::{setup_single_asset_coins, setup_test_client, Config},
    tx::{Address, AssetId, Input, Receipt, TxPointer, UtxoId, Word},
};

pub async fn setup_environment(
    coin: (Word, AssetId),
) -> (WalletUnlocked, WalletUnlocked, Vec<Input>) {
    let mut wallet = WalletUnlocked::new_random(None);
    let mut wallet2 = WalletUnlocked::new_random(None);
    // give the wallets the funds they need. might need to have simple token contract to do minting
    // Generate coins for wallet
    let all_coins: Vec<(UtxoId, Coin)> =
        setup_single_asset_coins(wallet.address(), coin.1, 1, coin.0);

    // Create the client and provider
    let mut provider_config = Config::local_node();
    provider_config.predicates = true;
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
    (wallet, wallet2, coin_inputs)
}

pub async fn make_order(
    wallet: &WalletUnlocked,
    gas_coin: Input,
    optional_inputs: &[Input],
    optional_outputs: &[Input],
) -> Vec<Receipt> {
    let mut tx = 
}
pub async fn take_order() -> Vec<Receipt> {}
