// fake_data.rs — generates random orders for testing
// uses only std library, no external crates needed

pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        SimpleRng { state: seed }
    }

    // basic LCG random number generator
    fn next(&mut self) -> u64 {
        self.state = self.state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    pub fn range(&mut self, min: u64, max: u64) -> u64 {
        min + (self.next() % (max - min + 1))
    }
}

use crate::order::{Order, OrderType, OrderSide};

pub fn generate_fake_orders(count: u64) -> Vec<Order> {
    let mut rng = SimpleRng::new(42); // seed 42 = same data every run (predictable)
    let mut orders = Vec::new();

    for id in 1..=count {
        let order_type = if rng.range(0, 4) == 0 {
            OrderType::Market       // ~20% market orders
        } else {
            OrderType::Limit        // ~80% limit orders
        };

        let side = if rng.range(0, 1) == 0 {
            OrderSide::Buy
        } else {
            OrderSide::Sell
        };

        // realistic BTC-style price range: $95,000 - $105,000
        let price = match order_type {
            OrderType::Market => 0,  // market orders have no price
            _                 => rng.range(95_000, 105_000),
        };

        let qty = rng.range(1, 20); // 1 to 20 units

        orders.push(Order::new(id, order_type, side, price, qty));
    }

    orders
}