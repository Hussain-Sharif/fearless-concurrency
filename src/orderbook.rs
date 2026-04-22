use crate::order::{Order,OrderSide};
use std::collections::BTreeMap;


pub fn aggregate_orders(orders:&Vec<Order>) -> BTreeMap<u64,u64>{
    let mut map:BTreeMap<u64, u64> = BTreeMap::new();

    for order in orders{
        let entry = map.entry(order.price).or_insert(0);

        *entry += order.qty;
    }
    map
}

// orderbook.rs — replace sort_calculate_best with this
pub fn calculate_best(aggregated_orders: &BTreeMap<u64, u64>, side: &OrderSide) -> u64 {
    match side {
        OrderSide::Buy  => aggregated_orders.keys().next_back().cloned().unwrap_or(0),
        OrderSide::Sell => aggregated_orders.keys().next().cloned().unwrap_or(0),
    }
}