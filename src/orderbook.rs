mod order;
use std::collections::BTreeMap;


pub fn aggregate_orders(orders:Vec<order::Order>) -> Vec<u64,u64>{
    let mut map:BTreeMap<u64, u64> = BTreeMap::new();

}

pub fn calculate_best(aggregated_orders:Vec<u64,u64>,side:order::OrderSide) -> (u64){

}