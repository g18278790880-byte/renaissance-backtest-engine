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
    pub fn can_cancel(&self) -> bool {
        match self {
            OrderStatus::New => true,
            OrderStatus::PartiallyFilled => true,
            OrderStatus::Filled => false,
            OrderStatus::Cancelled => false,
            OrderStatus::Rejected => false,
        }
    }
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

    pub fn cancel(&mut self) -> Result<(), String> {
        if self.status.can_cancel() {
            self.status = OrderStatus::Cancelled;
            Ok(())
        } else {
            Err(format!("order {} cannot be cancelled", self.id))
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
