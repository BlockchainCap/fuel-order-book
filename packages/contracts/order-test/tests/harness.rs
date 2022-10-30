mod utils {
    pub mod builder;
    pub mod environment;
}

mod success {
    use fuels::{test_helpers::DEFAULT_COIN_AMOUNT, tx::AssetId};

    use crate::utils::environment as env;

    #[tokio::test]
    async fn test_limit_order_predicate() {
        let coin = (DEFAULT_COIN_AMOUNT, AssetId::default());
        // might need to init another coin to correctly simulate the make/take
        let (maker, taker, coin_inputs) = env::setup_environment(coin).await;
    }
}

mod fail {

    #[tokio::test]
    async fn test_limit_order_predicate_wrong_take_coin() {}
}
