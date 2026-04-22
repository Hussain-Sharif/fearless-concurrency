use std::collections::BTreeMap;

use crate::order::{Order, OrderSide, Trade};


pub fn try_match_order(
    order_book_storage:&mut Vec<Order>, 
    agg_bids:&BTreeMap<u64,u64>, 
    agg_asks:& BTreeMap<u64,u64>) -> Option<Trade>{

    let best_bid_price = agg_bids.keys().next_back().cloned()?;
    let best_ask_price = agg_asks.keys().next().cloned()?;


    if best_bid_price < best_ask_price{
        return None;
    } 

    // finding the best bid & best ask order in orderbook storage whose qty > 0 (Not matched yet)
    let buy_idx = order_book_storage.iter().position(|o|{
        o.side==OrderSide::Buy && o.price == best_bid_price && o.qty > 0 
    })?;

    
    let sell_idx = order_book_storage.iter().position(|o|{
        o.side==OrderSide::Sell && o.price == best_ask_price && o.qty > 0 
    })?;


    // Fill quantity = minimum of the two orders
    let fill_qty = order_book_storage[buy_idx].qty
        .min(order_book_storage[sell_idx].qty);

    let trade = Trade {
        buy_order_id:  order_book_storage[buy_idx].id,
        sell_order_id: order_book_storage[sell_idx].id,
        fill_price:    best_ask_price, // trade happens at the resting order's price
        fill_qty,
    };

    // Reduce the quantities of the matched orders
    order_book_storage[buy_idx].qty -= fill_qty;
    order_book_storage[sell_idx].qty -= fill_qty;

    Some(trade)
}
