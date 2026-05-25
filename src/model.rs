pub struct Tick {
    pub symbol: String,
    pub price: i64,
    pub quantity: u64,
    pub timestamp: u64,
}

pub enum Side {
    Buy,
    Sell,
}

pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub side: Side,
    pub price: i64,
    pub quantity: u64,
}