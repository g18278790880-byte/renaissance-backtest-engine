use std::collections::BTreeMap;

use crate::model::Order;

#[derive(Debug)]
pub struct OrderBook {
    pub bids: BTreeMap<i64, Vec<Order>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.bids
            .entry(order.price)
            .or_insert(Vec::new())
            .push(order);
    }
}
