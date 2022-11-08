mod utils {
    pub mod builder;
    pub mod environment;
    pub mod order;
}

mod success {
    use std::str::FromStr;

    use crate::utils::builder::LimitOrder;

    use fuel_core_interfaces::common::fuel_crypto::SecretKey;
    use fuels::{
        client::FuelClient,
        prelude::{Bits256, Provider, Token, Tokenizable},
        signers::WalletUnlocked,
        test_helpers::DEFAULT_COIN_AMOUNT,
        tx::AssetId,
    };

    use crate::{utils::environment as env, utils::order as ord};

    #[tokio::test]
    async fn test_limit_order_predicate() {
        let coin = (DEFAULT_COIN_AMOUNT, AssetId::default());
        let (maker, taker, coin_inputs, provider) = env::setup_environment(coin).await;
        let order = LimitOrder {
            maker: maker.address().into(),
            maker_amount: coin.0,
            taker_amount: coin.0 / 2,
            maker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            taker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            salt: 42,
        };
        let (predicate, predicate_input_coin) = ord::create_order(&maker, &order, &provider).await;
        ord::verify_balance_of_maker_and_predicate(
            &maker,
            predicate.address(),
            coin.1,
            coin.0,
            &provider,
        )
        .await;
        ord::take_order(
            &taker,
            &order,
            &provider,
            predicate_input_coin,
            coin_inputs[0].clone(),
        )
        .await;
        ord::verify_balance_post_swap(&maker, &taker, predicate.address(), order, &provider).await;
    }

    #[tokio::test]
    async fn do_it_on_testnet() {
        // let (maker, taker, coin_inputs, provider) = env::setup_environment(coin).await;
        let fuel_client = FuelClient::from_str("https://node-beta-1.fuel.network/").unwrap();
        let provider = Provider::new(fuel_client);
        // let maker = WalletUnlocked::new_random(Some(provider.clone()));
        let maker_sk =
            SecretKey::from_str("b9ef66b50b59bcb5d5f23e24dac280d6f44e459360cb34f2c7dbe4749e05f5c4")
                .unwrap();
        let maker = WalletUnlocked::new_from_private_key(maker_sk, Some(provider.clone()));
    println!("{:?}", maker);
        let balance_maker = maker.get_balances().await.unwrap();
        println!("{:?}", balance_maker);
        let maker_sk =
            SecretKey::from_str("d9c951d48b532120a72ebf33270ada1aced7d5e3d87185e8a6114ab285837387")
                .unwrap();
        let taker = WalletUnlocked::new_from_private_key(maker_sk, Some(provider.clone()));
        let taker_token =
            "0x6dfc0f524d3006e103fc4bd733c4f012b5a089fd09361d44833cdf653e9afe5b".as_bytes();
        println!("{:?}", taker);
        let order = LimitOrder {
            maker: maker.address().into(),
            maker_amount: 100000,
            taker_amount: 300000,
            maker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            taker_token: Bits256::from_token(Token::B256(taker_token.slice)).unwrap(),
            salt: 42,
        };
        let (predicate, predicate_input_coin) =
            ord::create_order(&maker, &order, &provider.clone()).await;
        println!("{:?}", predicate.address());
        println!("{:?}", predicate_input_coin);
        ord::take_order(
            &taker,
            &order,
            &provider.clone(),
            predicate_input_coin,
            coin_inputs[0].clone(),
        )
        .await;
    }

    #[tokio::test]
    async fn take_the_order() {}
}

mod fail {
    use crate::utils::builder::LimitOrder;

    use fuels::{
        prelude::{Bits256, Token, Tokenizable},
        test_helpers::DEFAULT_COIN_AMOUNT,
        tx::AssetId,
    };

    use crate::{utils::environment as env, utils::order as ord};

    #[tokio::test]
    async fn test_limit_order_predicate_wrong_take_coin() {
        let coin = (DEFAULT_COIN_AMOUNT, AssetId::default());
        let (maker, taker, coin_inputs, provider) = env::setup_environment(coin).await;
        let order = LimitOrder {
            maker: maker.address().into(),
            maker_amount: coin.0,
            taker_amount: coin.0 / 2,
            maker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            taker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            salt: 42,
        };
        let (predicate, predicate_input_coin) = ord::create_order(&maker, &order, &provider).await;
        ord::verify_balance_of_maker_and_predicate(
            &maker,
            predicate.address(),
            coin.1,
            coin.0,
            &provider,
        )
        .await;
        ord::take_order(
            &taker,
            &order,
            &provider,
            predicate_input_coin,
            coin_inputs[0].clone(),
        )
        .await;
        ord::verify_balance_post_swap(&maker, &taker, predicate.address(), order, &provider).await;
    }
}
