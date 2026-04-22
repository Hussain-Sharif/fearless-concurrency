use std::collections::BTreeMap;

mod order;
mod orderbook;
mod fake_data;
mod engine;

fn incoming_order_validation(
    incoming_order: &order::Order,
    best_bid: u64,
    best_ask: u64,
) -> order::OrderDecision {
    if incoming_order.price == 0 && incoming_order.order_type != order::OrderType::Market {
        return order::OrderDecision::Reject;
    }
    if incoming_order.qty == 0 {
        return order::OrderDecision::Reject;
    }
    if incoming_order.order_type == order::OrderType::Market {
        return order::OrderDecision::Accept;
    }
    if incoming_order.order_type == order::OrderType::Limit
        && incoming_order.side == order::OrderSide::Buy
    {
        return if best_ask > 0 && incoming_order.price >= best_ask {
            order::OrderDecision::Accept
        } else {
            order::OrderDecision::Resting
        };
    }
    if incoming_order.order_type == order::OrderType::Limit
        && incoming_order.side == order::OrderSide::Sell
    {
        return if best_bid > 0 && incoming_order.price <= best_bid {
            order::OrderDecision::Accept
        } else {
            order::OrderDecision::Resting
        };
    }
    order::OrderDecision::Reject
}

fn main() {
    
    // generate 20 fake random orders — same data every run (seed=42)
    let incoming_test_data = fake_data::generate_fake_orders(20);

    let mut order_book_storage: Vec<order::Order> = Vec::new();
    let mut sorted_bids: BTreeMap<u64, u64> = BTreeMap::new();
    let mut sorted_asks: BTreeMap<u64, u64> = BTreeMap::new();
    let mut best_bid: u64 = 0;
    let mut best_ask: u64 = 0;

    let mut accept_count:  u32 = 0;
    let mut reject_count:  u32 = 0;
    let mut resting_count: u32 = 0;

    println!("------------------------------------------------");
    println!("   OrderBook Engine — Processing {} orders", incoming_test_data.len());
    println!("------------------------------------------------\n");

    for order in &incoming_test_data {
        let decision = incoming_order_validation(order, best_bid, best_ask);

        println!(
            "#{:02} | {:>5} {:>6} | price={:>7} qty={:>3} | bid={:>7} ask={:>7} | → {:?}",
            order.id,
            format!("{:?}", order.side),
            format!("{:?}", order.order_type),
            order.price,
            order.qty,
            best_bid,
            best_ask,
            decision
        );

        match &decision {
            order::OrderDecision::Accept | order::OrderDecision::Resting => {
                // Both accepted and resting orders go into the book
                // Rejected orders do NOT touch the book
                order_book_storage.push(order.clone());

                // recalculate best bid and ask after every new order
                let bids: Vec<order::Order> = order_book_storage
                    .iter()
                    .filter(|o| o.side == order::OrderSide::Buy)
                    .cloned()
                    .collect();

                let asks: Vec<order::Order> = order_book_storage
                    .iter()
                    .filter(|o| o.side == order::OrderSide::Sell)
                    .cloned()
                    .collect();

                let agg_bids = orderbook::aggregate_orders(&bids);
                let agg_asks = orderbook::aggregate_orders(&asks);

                (sorted_bids,best_bid) = orderbook::sort_calculate_best(&agg_bids, &order::OrderSide::Buy);
                (sorted_asks,best_ask) = orderbook::sort_calculate_best(&agg_asks, &order::OrderSide::Sell);

                // From here simply use matching engine to match and update the orderbook

                let mut current_best_bid = best_bid;
                let mut current_best_ask = best_ask;

                while(true){
                    if (best_bid > 0 && best_ask > 0) && (current_best_bid >= current_best_ask){
                        // match happening.

                        if !engine::try_match_order(&mut order_book_storage, &mut sorted_bids, &mut sorted_asks){
                            break;
                        }
                        
                        // remove orders with zero qty from order_book_storage, sorted_bids, and sorted_asks
                        


                    }else {
                        // resting.
                        break;
                    }
                } 

                match decision {
                    order::OrderDecision::Accept  => accept_count  += 1,
                    order::OrderDecision::Resting => resting_count += 1,
                    _ => {}
                }
            }
            order::OrderDecision::Reject => {
                reject_count += 1;
            }
        }
    }
   

    println!("\n================================================");
    println!("   Summary");
    println!("--------------------------------------------------");
    println!("  Accept:     {}", accept_count);
    println!("  Resting:    {}", resting_count);
    println!("  Reject:     {}", reject_count);
    println!("  Book size:  {} orders", order_book_storage.len());
    println!("  Final best_bid: ${}", best_bid);
    println!("  Final best_ask: ${}", best_ask);
    println!("================================================");
    

    

}
