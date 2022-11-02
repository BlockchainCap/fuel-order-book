use fuels::tx::{Address, Contract};

mod utils {
    pub mod builder;
    pub mod environment;
}

const PREDICATE: &str = "../order-predicate/out/debug/order-predicate.bin";
/// Gets the message to contract predicate
pub fn get_predicate() -> (Vec<u8>, Address) {
    let predicate_bytecode = std::fs::read(PREDICATE).unwrap();
    let predicate_root = Address::from(*Contract::root_from_code(&predicate_bytecode));
    (predicate_bytecode, predicate_root)
}
mod success {

    use crate::{get_predicate, utils::builder::LimitOrder};
    use fuels::{
        contract::predicate::Predicate,
        prelude::{Bech32Address, Bits256, Provider, Token, Tokenizable, TxParameters},
        signers::WalletUnlocked,
        test_helpers::DEFAULT_COIN_AMOUNT,
        tx::{AssetId, Input, TxPointer, UtxoId},
    };

    use crate::{utils::environment as env, PREDICATE};

    #[tokio::test]
    async fn test_limit_order_predicate() {
        let coin = (DEFAULT_COIN_AMOUNT, AssetId::default());
        // might need to init another coin to correctly simulate the make/take
        let (maker, taker, coin_inputs, provider) = env::setup_environment(coin).await;
        let predicate = Predicate::load_from(PREDICATE).unwrap();
        let (predicate_bytecode, predicate_root) = get_predicate();
        // create the order (fund the predicate)
        print_balances(&maker, &taker, predicate.address(), &provider, coin).await;
        let (_tx, _rec) = maker
            .transfer(predicate.address(), coin.0, coin.1, TxParameters::default())
            .await
            .unwrap();
        let predicate_coin = &provider
            .get_coins(&predicate_root.into(), AssetId::default())
            .await
            .unwrap()[0];
        let balance = maker.get_asset_balance(&coin.1).await.unwrap();
        let predicate_balance = provider
            .get_asset_balance(predicate.address(), coin.1)
            .await
            .unwrap();
        assert!(balance == 0);
        assert!(predicate_balance == coin.0);

        // this is the way i should do this, check the script. probably need to add some
        // inputs to this
        let order = LimitOrder {
            maker: maker.address().into(),
            maker_amount: coin.0,
            taker_amount: coin.0 / 2,
            maker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            taker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            salt: 42,
        };
        let predicate_coin_input = Input::CoinPredicate {
            utxo_id: UtxoId::from(predicate_coin.utxo_id.clone()),
            owner: predicate_root,
            amount: coin.0,
            asset_id: coin.1,
            tx_pointer: TxPointer::default(),
            maturity: 0,
            predicate: predicate_bytecode,
            predicate_data: vec![],
        };
        print_balances(&maker, &taker, predicate.address(), &provider, coin).await;
        let input_coins = &provider
            .get_coins(&taker.address(), AssetId::default())
            .await
            .unwrap()[0];
        let signed_coin_input = Input::CoinSigned {
            utxo_id: UtxoId::from(input_coins.utxo_id.clone()),
            owner: taker.address().into(),
            amount: input_coins.amount.clone().into(),
            asset_id: input_coins.asset_id.clone().into(),
            tx_pointer: TxPointer::default(),
            witness_index: 0,
            maturity: 0,
        };
        let _receipt = env::take_order(
            order,
            &taker,
            coin_inputs[0].clone(),
            predicate_coin_input,
            &vec![signed_coin_input],
            // &vec![],
            &vec![],
        )
        .await;
        // test these balances
        print_balances(&maker, &taker, predicate.address(), &provider, coin).await;
        let maker = maker.get_asset_balance(&coin.1).await.unwrap();
        let taker = taker.get_asset_balance(&coin.1).await.unwrap();
        assert!(maker == DEFAULT_COIN_AMOUNT / 2);
        assert!(taker == DEFAULT_COIN_AMOUNT);
        // assert!(pred_b == 0);
    }

    // for debugging purposes
    async fn print_balances(
        maker: &WalletUnlocked,
        taker: &WalletUnlocked,
        predicate_address: &Bech32Address,
        provider: &Provider,
        coin: (u64, AssetId),
    ) {
        let maker = maker.get_asset_balance(&coin.1).await.unwrap();
        let taker = taker.get_asset_balance(&coin.1).await.unwrap();
        let pred_b = provider
            .get_asset_balance(predicate_address, coin.1)
            .await
            .unwrap();
        println!("----------- COINS ----------");
        println!("Maker balance after: {:?}", maker);
        println!("Taker balance after: {:?}", taker);
        println!("Predicate balance after: {:?}", pred_b);
        println!("----------------------------");
    }
}

mod fail {

    #[tokio::test]
    async fn test_limit_order_predicate_wrong_take_coin() {}
}
