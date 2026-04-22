mod order;
mod orderbook;

fn incoming_order_validation(incoming_order:order::Order, best_bid:u64, best_ask:u64) -> order::OrderDecision{
    if incoming_order.price <= 0 || incoming_order.qty <=0 {
        return order::OrderDecision::REJECT;
    }

    if incoming_order.order_type ==  order::OrderType::LIMIT && incoming_order.price <= 0 {
        return order::OrderDecision::REJECT;
    }

    match incoming_order.order_type{
        order::OrderType::MARKET => {},
        order::OrderType::LIMIT => {},
        _ => return order::OrderDecision::REJECT
    }

    match incoming_order.side{
        order::OrderSide::BUY => {},
        order::OrderSide::SELL => {},
        _ => return order::OrderDecision::REJECT
    }

    if incoming_order.order_type == order::OrderType::MARKET{
        return order::OrderDecision::ACCEPT;
    }

    if incoming_order.order_type == order::OrderType::LIMIT 
    && incoming_order.price >= best_ask 
    && incoming_order.side == order::OrderSide::BUY {
        return order::OrderDecision::ACCEPT;
    }

    
    if incoming_order.order_type == order::OrderType::LIMIT 
    && incoming_order.price <= best_bid 
    && incoming_order.side == order::OrderSide::SELL {
        return order::OrderDecision::ACCEPT;
    }

    return order::OrderDecision::RESTING;
}

fn main() {
    let mut order_book_storage:Vec<order::Order> = Vec::new();

    let order1=order::Order::new(1,order::OrderType::LIMIT,order::OrderSide::BUY,100,10);
    let order2=order::Order::new(1,order::OrderType::LIMIT,order::OrderSide::BUY,90,15);
    let order3=order::Order::new(1,order::OrderType::MARKET,order::OrderSide::SELL,90,25);
    let order4=order::Order::new(1,order::OrderType::LIMIT,order::OrderSide::SELL,10,12);

    order_book_storage.push(order1);
    order_book_storage.push(order2);
    order_book_storage.push(order3);
    order_book_storage.push(order4);

    let mut best_bid:u64 = 0;
    let mut best_ask:u64 = 0;

    let aggregated_orders:Vec<u64,u64> = orderbook::aggregate_orders(orders);

    let mut (best_bid)

}
