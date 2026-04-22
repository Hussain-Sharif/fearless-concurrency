use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;

use crate::order::{Order, OrderSide, OrderType, OrderDecision, Trade};
use crate::orderbook;

pub struct EngineState {
    pub order_book: Vec<Order>,
    pub best_bid: u64,
    pub best_ask: u64,
    pub trades: Vec<Trade>,
}

impl EngineState {
    pub fn new() -> Self {
        EngineState {
            order_book: Vec::new(),
            best_bid: 0,
            best_ask: 0,
            trades: Vec::new(),
        }
    }
}

fn validate(order: &Order, best_bid: u64, best_ask: u64) -> OrderDecision {
    if order.qty == 0 {
        return OrderDecision::Reject;
    }

    if order.price == 0 && order.order_type != OrderType::Market {
        return OrderDecision::Reject;
    }

    if order.order_type == OrderType::Market {
        return OrderDecision::Accept;
    }

    if order.side == OrderSide::Buy {
        return if best_ask > 0 && order.price >= best_ask {
            OrderDecision::Accept
        } else {
            OrderDecision::Resting
        };
    }

    if order.side == OrderSide::Sell {
        return if best_bid > 0 && order.price <= best_bid {
            OrderDecision::Accept
        } else {
            OrderDecision::Resting
        };
    }

    OrderDecision::Reject
}

fn refresh_bests(state: &mut EngineState) {
    let bids: Vec<Order> = state.order_book
        .iter()
        .filter(|o| o.side == OrderSide::Buy && o.qty > 0)
        .cloned()
        .collect();

    let asks: Vec<Order> = state.order_book
        .iter()
        .filter(|o| o.side == OrderSide::Sell && o.qty > 0)
        .cloned()
        .collect();

    let agg_bids = orderbook::aggregate_orders(&bids);
    let agg_asks = orderbook::aggregate_orders(&asks);

    state.best_bid = orderbook::calculate_best(&agg_bids, &OrderSide::Buy);
    state.best_ask = orderbook::calculate_best(&agg_asks, &OrderSide::Sell);
}

fn try_match(state: &mut EngineState) -> Option<Trade> {
    let best_bid_price = state.order_book
        .iter()
        .filter(|o| o.side == OrderSide::Buy && o.qty > 0)
        .map(|o| o.price)
        .max()?;

    let best_ask_price = state.order_book
        .iter()
        .filter(|o| o.side == OrderSide::Sell && o.qty > 0)
        .map(|o| o.price)
        .min()?;

    if best_bid_price < best_ask_price {
        return None;
    }

    let buy_idx = state.order_book.iter().position(|o| {
        o.side == OrderSide::Buy && o.price == best_bid_price && o.qty > 0
    })?;

    let sell_idx = state.order_book.iter().position(|o| {
        o.side == OrderSide::Sell && o.price == best_ask_price && o.qty > 0
    })?;

    let fill_qty = state.order_book[buy_idx].qty.min(state.order_book[sell_idx].qty);

    let trade = Trade {
        buy_order_id: state.order_book[buy_idx].id,
        sell_order_id: state.order_book[sell_idx].id,
        fill_price: best_ask_price,
        fill_qty,
    };

    state.order_book[buy_idx].qty -= fill_qty;
    state.order_book[sell_idx].qty -= fill_qty;
    state.order_book.retain(|o| o.qty > 0);

    Some(trade)
}

pub fn run_engine(receiver: Receiver<Order>, shared_state: Arc<Mutex<EngineState>>) {
    println!("[Engine] started, waiting for orders...\n");

    for incoming in receiver {
        let mut state = shared_state.lock().unwrap();

        let decision = validate(&incoming, state.best_bid, state.best_ask);
        println!(
            "[Engine] #{} {:?} {:?} @ ${} → {:?}",
            incoming.id, incoming.side, incoming.order_type,
            incoming.price, decision
        );

        match decision {
            OrderDecision::Reject => {}

            OrderDecision::Accept | OrderDecision::Resting => {
                state.order_book.push(incoming);
                refresh_bests(&mut state);

                loop {
                    match try_match(&mut state) {
                        Some(trade) => {
                            println!(
                                "\n  [Engine] TRADE-MATCHED -> buy#{} x sell#{} qty={} @ ${}",
                                trade.buy_order_id, trade.sell_order_id,
                                trade.fill_qty, trade.fill_price
                            );
                            state.trades.push(trade);
                            refresh_bests(&mut state);
                        }
                        None => break,
                    }
                }
            }
        }
    }

    println!("\n[Engine] channel closed, shutting down.");
}