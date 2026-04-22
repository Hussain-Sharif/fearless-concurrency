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

pub fn sort_calculate_best(aggregated_orders:&BTreeMap<u64,u64>,side:&OrderSide) -> (BTreeMap<u64, u64>, u64){
    match side {
        OrderSide::Buy => {
            // sort the aggregated orders in descending order for bids
            let sorted_bids = aggregated_orders.iter().rev().map(|(price, qty)| (*price, *qty)).collect::<BTreeMap<u64, u64>>();
            let best_bid = match sorted_bids.iter().next(){
                Some((best_bid,_))=>*best_bid,
                None => 0
            };
            (sorted_bids,best_bid)
        },
        OrderSide::Sell => {
            // sort the aggregated orders in ascending order for asks
            let sorted_asks = aggregated_orders.iter().map(|(price, qty)| (*price, *qty)).collect::<BTreeMap<u64, u64>>();
            let best_ask = match sorted_asks.iter().next(){
                Some((best_ask,_))=>*best_ask,
                None => 0
            };
            (sorted_asks,best_ask)
        }
    }
}