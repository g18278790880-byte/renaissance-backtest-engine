#[derive(Debug)]
pub struct Tick {
    pub symbol: String,
    pub price: i64,
    pub quantity: u64,
    pub timestamp: u64,
}

#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
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
            OrderStatus::Filled => Some(OrderError::CannotCancelFilledOrder),
            OrderStatus::Cancelled => Some(OrderError::CannotCancelCancelledOrder),
            OrderStatus::Rejected => Some(OrderError::CannotCancelRejectedOrder),
        }
    }
}

#[derive(Debug)]
pub enum OrderError {
    CannotCancelFilledOrder,
    CannotCancelCancelledOrder,
    CannotCancelRejectedOrder,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Trade {
    pub trade_id: u64,
    pub order_id: u64,
    pub symbol: String,
    pub price: i64,
    pub quantity: u64,
    pub timestamp: u64,
}
