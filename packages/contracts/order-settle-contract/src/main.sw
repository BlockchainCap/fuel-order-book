contract;
use order::LimitOrder;

abi OrderSettler {
    fn take(order: LimitOrder);
    fn make(order: LimitOrder);
}

impl OrderSettler for Contract {
    fn take(order: LimitOrder) {
        
    }

    fn make(order: LimitOrder) {

    }

}
