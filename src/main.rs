mod order;
mod orderbook;
mod fake_data;
mod engine;

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("================================================");
    println!("   Concurrent OrderBook — 3 Threads");
    println!("================================================\n");

    // Arc  = multiple threads can co-own this value
    // Mutex = only one thread writes/reads at a time
    let shared_book = Arc::new(Mutex::new(engine::EngineState::new()));

    // mpsc channel: Thread 1 sends orders → Thread 2 receives them
    let (sender, receiver) = mpsc::channel::<order::Order>();

    // ── THREAD 2: Matching Engine ─────────────────────────────
    let book_for_engine = Arc::clone(&shared_book);
    let engine_thread = thread::spawn(move || {
        engine::run_engine(receiver, book_for_engine);
    });

    // ── THREAD 3: Display / Market Data ──────────────────────
    let book_for_display = Arc::clone(&shared_book);
    let display_thread = thread::spawn(move || {
        for i in 1..=8 {
            thread::sleep(Duration::from_millis(600));

            // lock → read → drop lock immediately so engine is not blocked long
            let state = book_for_display.lock().unwrap();
            let best_bid = state.best_bid;
            let best_ask = state.best_ask;
            let book_size = state.order_book.len();
            let trade_count = state.trades.len();
            drop(state);

            println!(
                "\n  [Display #{}] best_bid=${} | best_ask=${} | book={} orders | trades={}",
                i, best_bid, best_ask, book_size, trade_count
            );
        }
    });

    // ── THREAD 1 (main): Order Producer ──────────────────────
    let orders = fake_data::generate_fake_orders(20);
    for order in orders {
        println!(
            "\n[Producer] sending order #{} {:?} {:?} @ ${}",
            order.id, order.side, order.order_type, order.price
        );
        sender.send(order).unwrap();
        thread::sleep(Duration::from_millis(200));
    }

    drop(sender);

    engine_thread.join().unwrap();
    display_thread.join().unwrap();

    let final_state = shared_book.lock().unwrap();
    println!("\n================================================");
    println!("   Final Summary");
    println!("================================================");
    println!("  Orders in book : {}", final_state.order_book.len());
    println!("  Trades executed: {}", final_state.trades.len());
    println!("  Final best_bid : ${}", final_state.best_bid);
    println!("  Final best_ask : ${}", final_state.best_ask);
    println!("  --- Trades ---");
    for trade in &final_state.trades {
        println!(
            " buy#{} x sell#{} | qty={} @ ${}",
            trade.buy_order_id, trade.sell_order_id,
            trade.fill_qty, trade.fill_price
        );
    }
    println!("================================================");
}