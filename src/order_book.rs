use std::collections::{BTreeMap, HashMap};

use crate::model::{Order, OrderStatus, Side, Trade};

#[derive(Debug, PartialEq, Eq)]
pub struct DepthLevel {
    pub price: i64,
    pub total_quantity: u64,
    pub order_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderLocation {
    pub side: Side,
    pub price: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrderBookError {
    OrderNotFound(u64),
    DuplicateOrderId(u64),
}

#[derive(Debug)]
pub struct OrderBook {
    bids: BTreeMap<i64, Vec<Order>>,
    asks: BTreeMap<i64, Vec<Order>>,
    order_index: HashMap<u64, OrderLocation>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            order_index: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) -> Result<(), OrderBookError> {
        if self.order_index.contains_key(&order.id) {
            return Err(OrderBookError::DuplicateOrderId(order.id));
        }

        let location = OrderLocation {
            side: order.side,
            price: order.price,
        };

        self.order_index.insert(order.id, location);

        match order.side {
            Side::Buy => {
                self.bids.entry(order.price).or_default().push(order);
            }
            Side::Sell => {
                self.asks.entry(order.price).or_default().push(order);
            }
        }

        Ok(())
    }

    pub fn cancel_order(&mut self, order_id: u64) -> Result<Order, OrderBookError> {
        let location = self
            .order_index
            .remove(&order_id)
            .ok_or(OrderBookError::OrderNotFound(order_id))?;

        let book_side = match location.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        let orders = book_side
            .get_mut(&location.price)
            .ok_or(OrderBookError::OrderNotFound(order_id))?;

        let position = orders
            .iter()
            .position(|order| order.id == order_id)
            .ok_or(OrderBookError::OrderNotFound(order_id))?;

        let cancelled_order = orders.remove(position);

        if orders.is_empty() {
            book_side.remove(&location.price);
        }

        Ok(cancelled_order)
    }

    pub fn best_bid(&self) -> Option<i64> {
        self.bids.keys().next_back().copied()
    }

    pub fn best_ask(&self) -> Option<i64> {
        self.asks.keys().next().copied()
    }

    pub fn spread(&self) -> Option<i64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    pub fn bid_depth(&self, levels: usize) -> Vec<DepthLevel> {
        self.bids
            .iter()
            .rev()
            .take(levels)
            .map(|(&price, orders)| DepthLevel {
                price,
                total_quantity: orders.iter().map(|order| order.quantity).sum(),
                order_count: orders.len(),
            })
            .collect()
    }

    pub fn ask_depth(&self, levels: usize) -> Vec<DepthLevel> {
        self.asks
            .iter()
            .take(levels)
            .map(|(&price, orders)| DepthLevel {
                price,
                total_quantity: orders.iter().map(|order| order.quantity).sum(),
                order_count: orders.len(),
            })
            .collect()
    }

    pub fn order_count(&self) -> usize {
        self.order_index.len()
    }

    pub fn bid_order_count(&self) -> usize {
        self.bids.values().map(|orders| orders.len()).sum()
    }

    pub fn ask_order_count(&self) -> usize {
        self.asks.values().map(|orders| orders.len()).sum()
    }

    pub fn contains_order(&self, order_id: u64) -> bool {
        self.order_index.contains_key(&order_id)
    }

    pub fn get_order(&self, order_id: u64) -> Option<&Order> {
        let location = self.order_index.get(&order_id)?;

        let book_side = match location.side {
            Side::Buy => &self.bids,
            Side::Sell => &self.asks,
        };

        let orders = book_side.get(&location.price)?;

        orders.iter().find(|order| order.id == order_id)
    }

    pub fn best_bid_order(&self) -> Option<&Order> {
        let best_bid = self.best_bid()?;

        self.bids.get(&best_bid)?.first()
    }

    pub fn best_ask_order(&self) -> Option<&Order> {
        let best_ask = self.best_ask()?;

        self.asks.get(&best_ask)?.first()
    }

    pub fn match_best_orders(&mut self, trade_id: u64, timestamp: u64) -> Option<Trade> {
        let bid_price = self.best_bid()?;
        let ask_price = self.best_ask()?;

        if bid_price < ask_price {
            return None;
        }

        let (trade, filled_buy_order_id, filled_sell_order_id) = {
            let buy_orders = self.bids.get_mut(&bid_price)?;
            let sell_orders = self.asks.get_mut(&ask_price)?;

            let buy_order = buy_orders.first_mut()?;
            let sell_order = sell_orders.first_mut()?;

            let traded_quantity = buy_order.quantity.min(sell_order.quantity);

            let trade = Trade {
                trade_id,
                buy_order_id: buy_order.id,
                sell_order_id: sell_order.id,
                symbol: buy_order.symbol.clone(),
                price: ask_price,
                quantity: traded_quantity,
                timestamp,
            };

            buy_order.quantity -= traded_quantity;
            sell_order.quantity -= traded_quantity;

            buy_order.status = if buy_order.quantity == 0 {
                OrderStatus::Filled
            } else {
                OrderStatus::PartiallyFilled
            };

            sell_order.status = if sell_order.quantity == 0 {
                OrderStatus::Filled
            } else {
                OrderStatus::PartiallyFilled
            };

            let filled_buy_order_id = if buy_order.quantity == 0 {
                Some(buy_order.id)
            } else {
                None
            };

            let filled_sell_order_id = if sell_order.quantity == 0 {
                Some(sell_order.id)
            } else {
                None
            };

            (trade, filled_buy_order_id, filled_sell_order_id)
        };

        if let Some(order_id) = filled_buy_order_id {
            if let Some(orders) = self.bids.get_mut(&bid_price) {
                orders.remove(0);

                if orders.is_empty() {
                    self.bids.remove(&bid_price);
                }
            }

            self.order_index.remove(&order_id);
        }

        if let Some(order_id) = filled_sell_order_id {
            if let Some(orders) = self.asks.get_mut(&ask_price) {
                orders.remove(0);

                if orders.is_empty() {
                    self.asks.remove(&ask_price);
                }
            }

            self.order_index.remove(&order_id);
        }

        Some(trade)
    }

    pub fn match_orders(&mut self, start_trade_id: u64, timestamp: u64) -> Vec<Trade> {
        let mut trades = Vec::new();
        let mut next_trade_id = start_trade_id;

        while let Some(trade) = self.match_best_orders(next_trade_id, timestamp) {
            trades.push(trade);
            next_trade_id += 1;
        }

        trades
    }
}

#[cfg(test)]
mod tests;
