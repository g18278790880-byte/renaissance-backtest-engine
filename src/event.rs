use crate::model::{OrderRequest, OrderUpdate, Tick, Trade};

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    MarketTick(Tick),
    OrderRequest(OrderRequest),
    OrderUpdate(OrderUpdate),
    Trade(Trade),
}

impl Event {
    pub fn event_type(&self) -> &'static str {
        match self {
            Event::MarketTick(_) => "market_tick",
            Event::OrderRequest(_) => "order_request",
            Event::OrderUpdate(_) => "order_update",
            Event::Trade(_) => "trade",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{OrderRequest, Side, Tick};

    #[test]
    fn event_type_returns_market_tick() {
        let event = Event::MarketTick(Tick {
            symbol: String::from("BTCUSDT"),
            price: 100_000,
            quantity: 1,
            timestamp: 1_717_000_000,
        });

        assert_eq!(event.event_type(), "market_tick");
    }

    #[test]
    fn event_type_returns_order_request() {
        let event = Event::OrderRequest(OrderRequest {
            symbol: String::from("BTCUSDT"),
            side: Side::Buy,
            price: 100_000,
            quantity: 1,
        });

        assert_eq!(event.event_type(), "order_request");
    }
}
