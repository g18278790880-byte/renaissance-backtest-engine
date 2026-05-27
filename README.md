# Renaissance Backtest Engine

> 一个基于 Rust 的事件驱动交易回测与订单管理引擎，从核心交易模型开始逐步构建。

Renaissance Backtest Engine 是一个面向交易系统工程实践的 Rust 项目。它从行情、订单、成交、订单簿等基础模型出发，逐步扩展为一个事件驱动的回测引擎，覆盖策略执行、撮合模拟、持仓统计、API、日志、指标和性能测试等能力。

当前项目已完成订单簿模块中的核心查询能力：已经定义了第一批核心市场与订单模型，支持基础订单状态流转，能够按买卖方向保存订单，并查询最优买价、最优卖价、买卖价差和盘口深度。

---

## 项目目标

本项目的长期目标是构建一个迷你版交易基础设施：能够回放行情数据、更新市场状态、驱动策略决策、管理订单、模拟成交，并输出回测结果。

```text
market-data-simulator
        ↓
event-bus / channel
        ↓
order-book
        ↓
strategy-engine
        ↓
order-manager
        ↓
execution-simulator
        ↓
portfolio + backtest-report
```

项目重点关注交易系统背后的工程问题：

- 使用 Rust 类型系统建模 `Tick`、`Order`、`Trade`、持仓和订单状态；
- 维护可预测、可测试的价格档位订单簿；
- 用单元测试验证订单生命周期和订单簿行为；
- 通过事件模型连接行情、策略、订单和成交；
- 在核心逻辑稳定后补充 API、日志、指标、存储和性能测试。

---

## 当前进展

已经实现：

- Rust 项目骨架，包含 `main.rs`、`model.rs` 和 `order_book.rs`；
- 核心数据模型：
  - `Tick`
  - `Order`
  - `Trade`
  - `Side`
  - `OrderStatus`
  - `OrderError`
- 订单生命周期逻辑：
  - `Order::fill()`
  - `Order::cancel()`
  - 对已成交、已取消、已拒绝订单的取消校验；
- 初版双边 `OrderBook`：
  - 使用 `BTreeMap<i64, Vec<Order>>` 按价格档位分别保存买盘和卖盘订单；
  - 支持 `add_order()` 添加订单；
  - 支持 `best_bid()` 查询当前最高买价；
  - 支持 `best_ask()` 查询当前最低卖价；
  - 支持 `spread()` 计算买卖价差；
  - 支持 `bid_depth()` 聚合买盘深度；
  - 支持 `ask_depth()` 聚合卖盘深度；
- 单元测试覆盖订单取消规则、最优买卖价、价差和盘口深度查询。

当前里程碑：

```text
Module 2：订单簿
状态：核心查询能力已完成
最新完成：add_order() / best_bid() / best_ask() / spread() / bid_depth() / ask_depth()
```

---

## 已实现示例

当前可执行程序会构建一个简单的双边订单簿，并输出最优买卖价、价差和盘口深度：

```rust
let mut order_book = OrderBook::new();

order_book.add_order(Order {
    id: 1,
    symbol: String::from("BTCUSDT"),
    side: Side::Buy,
    price: 100_000,
    quantity: 1,
    status: OrderStatus::New,
});

order_book.add_order(Order {
    id: 2,
    symbol: String::from("BTCUSDT"),
    side: Side::Buy,
    price: 100_000,
    quantity: 2,
    status: OrderStatus::New,
});

order_book.add_order(Order {
    id: 3,
    symbol: String::from("BTCUSDT"),
    side: Side::Sell,
    price: 101_000,
    quantity: 3,
    status: OrderStatus::New,
});

assert_eq!(order_book.best_bid(), Some(100_000));
assert_eq!(order_book.best_ask(), Some(101_000));
assert_eq!(order_book.spread(), Some(1_000));
```

订单簿使用两个 `BTreeMap` 分别保存买盘和卖盘。买盘最优价取最高价格，卖盘最优价取最低价格：

```rust
pub fn best_bid(&self) -> Option<i64> {
    self.bids.keys().next_back().copied()
}

pub fn best_ask(&self) -> Option<i64> {
    self.asks.keys().next().copied()
}
```

`spread()` 在买卖两边都存在时返回 `best_ask - best_bid`；任意一边为空时返回 `None`，避免使用特殊价格作为哨兵值。

深度查询会按价格档位聚合同一价格下的订单数量：

```rust
pub struct DepthLevel {
    pub price: i64,
    pub total_quantity: u64,
    pub order_count: usize,
}
```

---

## 架构路线

计划中的模块结构：

```text
src/
├── main.rs          # 可执行入口与当前功能演示
├── model.rs         # Tick / Order / Trade / Position 等核心模型
├── event.rs         # 连接行情、策略、订单和成交的事件枚举
├── order_book.rs    # 价格档位订单簿
├── strategy.rs      # 策略 trait 与示例策略
├── execution.rs     # 撮合、手续费、滑点模拟
├── backtest.rs      # 事件回放循环与回测报告生成
├── portfolio.rs     # 持仓、现金、PnL 统计
├── api.rs           # Axum HTTP API
├── metrics.rs       # 运行指标与回测指标
└── storage.rs       # SQLite / SQLx 持久化
```

计划中的 API：

```text
GET  /health
GET  /orders
GET  /positions
POST /backtests
GET  /backtests/{id}
GET  /metrics
```

---

## 进度概览

| 模块 | 状态 | 说明 |
| --- | --- | --- |
| 项目骨架 | 已完成 | Cargo 项目和基础模块已建立 |
| 核心模型 | 已完成 | `Tick`、`Order`、`Trade`、方向、状态、错误枚举 |
| 订单生命周期 | 已完成 | 已实现成交和取消逻辑，并配有单元测试 |
| 双边订单簿 | 已完成 | 已实现买盘 `bids` 和卖盘 `asks` |
| 最优价查询 | 已完成 | 已实现 `best_bid()` 和 `best_ask()` |
| 价差查询 | 已完成 | 已实现 `spread()` |
| 盘口深度查询 | 已完成 | 已实现 `bid_depth()` 和 `ask_depth()` |
| 订单簿内取消订单 | 未开始 | 后续支持从订单簿中移除或更新订单 |
| 策略接口 | 未开始 | 后续使用 trait 抽象策略 |
| 事件循环 | 未开始 | 后续连接行情、订单和成交事件 |
| 回测引擎 | 未开始 | 后续实现事件回放、撮合模拟和报告输出 |
| API / 指标 / 存储 | 未开始 | 后续补充工程化能力 |

---

## 技术栈

| 类型 | 技术 |
| --- | --- |
| 核心语言 | Rust |
| 异步运行时 | Tokio |
| HTTP API | Axum |
| 序列化 | Serde / serde_json |
| 行情数据 | CSV / Polars |
| 存储 | SQLite / SQLx |
| 日志 | tracing / tracing-subscriber |
| 指标 | Prometheus 风格指标 |
| 性能测试 | Criterion |
| 测试 | Rust built-in test framework |

---

## 本地运行

运行当前示例：

```bash
cargo run
```

当前示例会：

- 创建一个内存中的订单簿；
- 插入多笔买单和卖单；
- 输出当前最高买价、最低卖价和买卖价差；
- 输出买盘和卖盘的价格档位深度。

运行测试：

```bash
cargo test
```

格式化与检查：

```bash
cargo fmt
cargo check
```

---

## 下一步

订单簿模块的核心查询能力已经完成。接下来会继续补齐：

1. 支持在订单簿中取消或移除订单；
2. 为订单簿增加更完整的边界条件测试；
3. 引入事件模型，连接行情、订单和成交；
4. 设计策略接口，为后续回测循环做准备。

订单簿进一步稳定后，项目会进入事件模型和策略执行模块。

---

## 项目定位

这个仓库用于展示 Rust 在交易系统类问题中的工程实践：

- 类型驱动的领域建模；
- 明确、可测试的状态流转；
- 逐步构建的市场微观结构组件；
- 从小模块验证开始，逐渐扩展到异步服务、API、指标和存储。
