use fuels::tx::{Address, Contract};

mod utils {
    pub mod builder;
    pub mod environment;
}

const PREDICATE: &str = "../order-predicate/out/debug/order-predicate.bin";
/// Gets the message to contract predicate
pub async fn get_contract_message_predicate() -> (Vec<u8>, Address) {
    let predicate_bytecode = std::fs::read(PREDICATE).unwrap();
    let predicate_root = Address::from(*Contract::root_from_code(&predicate_bytecode));
    (predicate_bytecode, predicate_root)
}
mod success {
    abigen!(
        Order,
        "packages/contracts/order-settle-contract/out/debug/order-settle-contract-abi.json"
    );

    use fuels::{
        contract::predicate::Predicate,
        prelude::{abigen, Bits256, TxParameters},
        test_helpers::DEFAULT_COIN_AMOUNT,
        tx::AssetId,
    };

    use crate::{utils::environment as env, PREDICATE};

    #[tokio::test]
    async fn test_limit_order_predicate() {
        let coin = (DEFAULT_COIN_AMOUNT, AssetId::default());
        // might need to init another coin to correctly simulate the make/take
        let (maker, taker, coin_inputs, provider) = env::setup_environment(coin).await;

        let predicate = Predicate::load_from(PREDICATE).unwrap();

        // let (_bytecode, predicate_root) = get_contract_message_predicate().await;
        // let (tx, receipt) =
        //     env::send_coins_to_predicate(&maker, predicate.address(), coin.0, coin.1).await;

        let (tx, rec) = maker
            .transfer(predicate.address(), coin.0, coin.1, TxParameters::default())
            .await
            .unwrap();

        let balance = maker.get_asset_balance(&coin.1).await.unwrap();
        let predicate_balance = provider
            .get_asset_balance(predicate.address(), coin.1)
            .await
            .unwrap();
        assert!(balance == 0);
        assert!(predicate_balance == coin.0);

        let _receipt = env::take_order(&taker, coin_inputs[0].clone(), &vec![], &vec![]).await;
        //
        // 
        // 
        // 
        // 
        //  below here is drafting, doesnt actually work, above we use a script


        // spend the predicate with taker
        let make_coin: [u8; 32] = coin.1.into();
        let take_coin: [u8; 32] = coin.1.into();
        // currently cant pass any friendly data in the predicate
        // data, so will just pass a byte array
        let order = LimitOrder {
            maker_token: Bits256(make_coin),
            taker_token: Bits256(take_coin),
            maker_amount: coin.0,
            taker_amount: coin.0,
            // taker_token_fee: 0,
            // maker: maker,
            // taker: taker,
            // sender
            salt: 42,
        };

        // major hackage here to get this to work with poor support for predicates
        // in the SDK
        let mut predicate_data = vec![];
        predicate_data.push(make_coin.to_vec());
        predicate_data.push(take_coin.to_vec());
        let maker_amount: [u8; 8] = coin.0.to_be_bytes();
        let taker_amount: [u8; 8] = coin.0.to_be_bytes();
        predicate_data.push(maker_amount.to_vec());
        predicate_data.push(taker_amount.to_vec());

        let raw_arr = predicate_data.into_iter().flatten().collect();
        taker
            .receive_from_predicate(
                predicate.address(),
                predicate.code(),
                coin.0,
                coin.1,
                Some(raw_arr),
                TxParameters::default(),
            )
            .await
            .unwrap();

        // test these balances
        let maker = maker.get_asset_balance(&coin.1).await.unwrap();
        let taker = taker.get_asset_balance(&coin.1).await.unwrap();
        println!("{:?}\n{:?}", maker, taker)
    }
}

mod fail {

    #[tokio::test]
    async fn test_limit_order_predicate_wrong_take_coin() {}
}
