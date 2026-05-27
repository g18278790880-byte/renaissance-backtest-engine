use crate::event::Event;

pub fn handle_event(event: &Event) {
    match event {
        Event::MarketTick(tick) => {
            println!(
                "market tick received: symbol={}, price={}, quantity={}",
                tick.symbol, tick.price, tick.quantity
            );
        }
        Event::OrderRequest(request) => {
            println!(
                "order request received: symbol={}, side={:?}, price={}, quantity={}",
                request.symbol, request.side, request.price, request.quantity
            );
        }
        Event::OrderUpdate(update) => {
            println!(
                "order update received: order_id={}, status={:?}, filled={}, remaining={}",
                update.order_id, update.status, update.filled_quantity, update.remaining_quantity
            );
        }
        Event::Trade(trade) => {
            println!(
                "trade received: buy_order_id={}, sell_order_id={}, price={}, quantity={}",
                trade.buy_order_id, trade.sell_order_id, trade.price, trade.quantity
            );
        }
    }
}
