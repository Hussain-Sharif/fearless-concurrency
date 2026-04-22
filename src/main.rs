use std::collections::BTreeMap;

use crate::order::Trade;

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
                order_book_storage.push(order.clone());

                loop {
                    // Recalculate after every match attempt
                    let bids: Vec<order::Order> = order_book_storage.iter()
                        .filter(|o| o.side == order::OrderSide::Buy && o.qty > 0)
                        .cloned().collect();
                    let asks: Vec<order::Order> = order_book_storage.iter()
                        .filter(|o| o.side == order::OrderSide::Sell && o.qty > 0)
                        .cloned().collect();

                    let agg_bids = orderbook::aggregate_orders(&bids);
                    let agg_asks = orderbook::aggregate_orders(&asks);

                    best_bid = orderbook::calculate_best(&agg_bids, &order::OrderSide::Buy);
                    best_ask = orderbook::calculate_best(&agg_asks, &order::OrderSide::Sell);

                    // Only try matching if spread has crossed
                    if best_bid > 0 && best_ask > 0 && best_bid >= best_ask {
                        
                        match engine::try_match_order(&mut order_book_storage, &agg_bids, &agg_asks) {
                            Some(trade) => {
                                
                                println!(
                                    " TRADE-MATCHED -> buy#{} X sell#{} | qty={} @ ${}",
                                    trade.buy_order_id, trade.sell_order_id,
                                    trade.fill_qty, trade.fill_price
                                );


                                // Remove fully filled orders (qty == 0) from book
                                order_book_storage.retain(|o| o.qty > 0);
                            }
                            None => break, // no more matches possible
                        }
                    } else {
                        break; // spread not crossed, nothing to match
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
