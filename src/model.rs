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
