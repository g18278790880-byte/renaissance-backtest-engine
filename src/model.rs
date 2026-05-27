#[derive(Debug)]
pub struct Tick {
    pub symbol: String,
    pub price: i64,
    pub quantity: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

impl OrderStatus {
    pub fn cancel_error(&self) -> Option<OrderError> {
        match self {
            OrderStatus::New => None,
            OrderStatus::PartiallyFilled => None,
            OrderStatus::Filled => Some(OrderError::Filled),
            OrderStatus::Cancelled => Some(OrderError::Cancelled),
            OrderStatus::Rejected => Some(OrderError::Rejected),
        }
    }
}

#[derive(Debug)]
pub enum OrderError {
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub side: Side,
    pub price: i64,
    pub quantity: u64,
    pub status: OrderStatus,
}

impl Order {
    pub fn fill(&mut self) {
        self.status = OrderStatus::Filled;
    }

    pub fn cancel(&mut self) -> Result<(), OrderError> {
        match self.status.cancel_error() {
            None => {
                self.status = OrderStatus::Cancelled;
                Ok(())
            }
            Some(err) => Err(err),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Trade {
    pub trade_id: u64,
    pub buy_order_id: u64,
    pub sell_order_id: u64,
    pub symbol: String,
    pub price: i64,
    pub quantity: u64,
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_order_can_be_cancelled() {
        let mut order = Order {
            id: 1,
            symbol: String::from("BTCUSDT"),
            side: Side::Buy,
            price: 100_000,
            quantity: 1,
            status: OrderStatus::New,
        };

        let result = order.cancel();

        assert!(result.is_ok());
        assert!(matches!(order.status, OrderStatus::Cancelled));
    }

    #[test]
    fn filled_order_cannot_be_cancelled() {
        let mut order = Order {
            id: 1,
            symbol: String::from("BTCUSDT"),
            side: Side::Buy,
            price: 100_000,
            quantity: 1,
            status: OrderStatus::New,
        };

        order.fill();
        let result = order.cancel();

        assert!(result.is_err());
        assert!(matches!(result, Err(OrderError::Filled)));
    }
}
