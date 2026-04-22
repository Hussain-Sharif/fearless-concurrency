# fearless-concurrency 🦀

A concurrent limit order book matching engine built in Rust — my first real systems project.

Wanted to understand how CEX order books actually work under the hood, 
so I built one from scratch using Rust's multithreading primitives.
No external crates. Just `std`.

## What it does

Three threads running simultaneously:

- **Producer** (main thread) — generates fake limit/market orders and sends them over a channel
- **Matching Engine** (thread 2) — receives orders, validates them, adds to the book, matches when bid >= ask
- **Market Data Display** (thread 3) — reads the book every 600ms and prints live best bid/ask snapshots

## How orders flow

```text
Producer ──[mpsc channel]──▶ Engine Thread
│
Arc<Mutex<EngineState>>
│
Display Thread (read-only snapshots)
```


## Concepts used

- `Arc<Mutex<T>>` — shared state across threads safely
- `std::sync::mpsc` — message passing between threads  
- `BTreeMap<u64, u64>` — sorted price levels (best bid/ask in O(log n))
- Price-time priority matching — highest bid matches lowest ask first
- RAII lock release — Mutex drops automatically, no manual unlock

## Project structure
```text
src/
├── main.rs ← thread spawning and wiring
├── order.rs ← Order, OrderType, OrderSide, Trade structs
├── orderbook.rs ← aggregate orders, calculate best bid/ask
├── engine.rs ← EngineState + matching logic (runs on Thread 2)
└── fake_data.rs ← seeded random order generator (no external crates)
```


## Run it

```bash
git clone https://github.com/Hussain-Sharif/fearless-concurrency
cd fearless-concurrency
cargo run
```

## What I learned

Threading in Rust clicked for me when I stopped thinking about it as 
"running code in parallel" and started thinking about it as 
**who owns what data and when**.

`Arc` answers *who can see it*. `Mutex` answers *who can touch it right now*.
Once that made sense, the rest followed.

---

Built while going through the Solana Fellowship bootcamp — Week 3 covers CEX order books.  
This is me taking that concept and making it actually run.
