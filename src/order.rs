

#[derive(Clone, Debug, PartialEq)]
pub enum OrderType {
    Limit,  
    Market,
    Stop,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OrderDecision {
    Accept,
    Reject,
    Resting,  
}

#[derive(Clone, Debug)]
pub struct Order {
    pub id:         u64,
    pub order_type: OrderType,
    pub side:       OrderSide,
    pub price:      u64,
    pub qty:        u64,
}

impl Order {
    pub fn new(id: u64, order_type: OrderType, side: OrderSide, price: u64, qty: u64) -> Self {
        Order { id, order_type, side, price, qty }
    }
}

#[derive(Clone, Debug)]
pub struct Trade {
    pub buy_order_id:  u64,
    pub sell_order_id: u64,
    pub fill_price:    u64,
    pub fill_qty:      u64,
}