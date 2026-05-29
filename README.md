# Renaissance Backtest Engine

> 用 Rust 构建的事件驱动交易回测与订单管理系统雏形。

Renaissance Backtest Engine 是一个从零实现的迷你交易系统项目。它目前已经完成核心领域模型、双边订单簿、基础撮合、事件引擎和策略接口，可以跑通一条同步版事件驱动链路：

```text
MarketTick
    ↓
Strategy
    ↓
OrderRequest
    ↓
Engine
    ↓
OrderBook
    ↓
Trade / OrderUpdate
```

项目目标不是做一个完整交易所，而是用 Rust 逐步构建交易系统的核心组件：类型建模、订单簿、事件流、策略接口、撮合、回测循环，以及后续的异步消息通道、API、指标和存储。

---

## Current Status

当前进度：**Module 3 已完成：策略接口与事件驱动模型**。

已完成能力：

- 核心交易模型：`Tick`、`OrderRequest`、`Order`、`OrderUpdate`、`Trade`
- 事件模型：`Event::MarketTick`、`Event::OrderRequest`、`Event::OrderUpdate`、`Event::Trade`
- 双边订单簿：买盘、卖盘、最优价、价差、盘口深度
- 订单索引：按订单 ID 查询、取消、去重
- 基础撮合：价格交叉时生成成交，并更新订单状态
- 事件引擎：处理订单请求，输出 `Trade` 和 `OrderUpdate`
- 策略接口：`Strategy` trait
- 示例策略：`ThresholdStrategy`
- 同步 tick 处理：`Engine::process_market_tick()`
- 测试整理：`order_book` 和 `engine` 测试已拆分到独立测试文件

验证状态：

```text
cargo test: 40 passed
```

当前仍有少量 `dead_code` / 未使用 warning，主要来自后续模块会继续使用的接口，例如 `Rejected`、`on_order_update()` 和部分查询方法。

---

## Architecture

当前核心链路：

```text
+-------------+
| Market Tick |
+------+------+ 
       |
       v
+-------------+
|  Strategy   |
| on_tick()   |
+------+------+ 
       |
       v
+----------------+
| OrderRequest   |
+-------+--------+
        |
        v
+----------------+
| Engine         |
| handle_event() |
+-------+--------+
        |
        v
+----------------+
| OrderBook      |
| bids / asks    |
+-------+--------+
        |
        v
+----------------------+
| Trade / OrderUpdate  |
+----------------------+
```

当前代码结构：

```text
src/
├── main.rs              # 当前同步事件流演示
├── model.rs             # Tick / Order / Trade / OrderUpdate 等核心模型
├── event.rs             # Event 枚举
├── engine.rs            # 事件处理引擎
├── engine/
│   └── tests.rs         # Engine 单元测试
├── strategy.rs          # Strategy trait 与 ThresholdStrategy
├── order_book.rs        # 双边订单簿与基础撮合
└── order_book/
    └── tests.rs         # OrderBook 单元测试，按行为分组
```

---

## Core Modules

### `model.rs`

定义交易系统中的基础数据结构：

- `Tick`：行情 tick
- `OrderRequest`：外部订单请求
- `Order`：订单簿内部订单
- `OrderUpdate`：订单状态变化
- `Trade`：成交记录
- `Side`：买卖方向
- `OrderStatus`：订单状态
- `OrderError`：订单状态错误

### `event.rs`

定义系统内部事件：

```rust
pub enum Event {
    MarketTick(Tick),
    OrderRequest(OrderRequest),
    OrderUpdate(OrderUpdate),
    Trade(Trade),
}
```

### `order_book.rs`

实现双边订单簿：

- `add_order()`
- `cancel_order()`
- `best_bid()`
- `best_ask()`
- `spread()`
- `bid_depth()`
- `ask_depth()`
- `contains_order()`
- `get_order()`
- `best_bid_order()`
- `best_ask_order()`
- `match_best_orders()`
- `match_orders()`

订单簿使用：

- `BTreeMap<i64, Vec<Order>>` 维护价格档位
- `HashMap<u64, OrderLocation>` 维护订单 ID 索引

### `engine.rs`

`Engine` 是当前系统的事件协调层：

- 接收 `Event`
- 将 `OrderRequest` 转换为内部 `Order`
- 调用 `OrderBook` 添加订单并尝试撮合
- 输出 `Event::Trade`
- 输出 `Event::OrderUpdate`
- 通过 `process_market_tick()` 串联策略和订单引擎

### `strategy.rs`

定义策略接口：

```rust
pub trait Strategy {
    fn on_tick(&mut self, tick: &Tick) -> Vec<OrderRequest>;

    fn on_order_update(&mut self, _update: &OrderUpdate) {}
}
```

当前示例策略是 `ThresholdStrategy`：

- 价格低于 `buy_below`：生成买单
- 价格高于 `sell_above`：生成卖单
- 其他情况：不交易

---

## Example

当前 `main.rs` 演示的是同步版 tick → strategy → engine 流程：

```rust
let mut engine = Engine::new();
let mut strategy = ThresholdStrategy::new(String::from("BTCUSDT"), 99_000, 101_000, 1);

let tick = Tick {
    symbol: String::from("BTCUSDT"),
    price: 98_000,
    quantity: 1,
    timestamp: 1_717_000_000,
};

let output_events = engine.process_market_tick(&tick, &mut strategy).unwrap();
```

当前示例会：

- 输入一条 `Tick`
- 由 `ThresholdStrategy` 生成 `OrderRequest`
- 交给 `Engine`
- 写入 `OrderBook`
- 输出事件列表和当前盘口状态

---

## Getting Started

运行示例：

```bash
cargo run
```

运行测试：

```bash
cargo test
```

格式化：

```bash
cargo fmt
```

静态检查：

```bash
cargo check
```

---

## Test Coverage

当前测试覆盖：

- `model`：订单取消、订单请求转换
- `event`：事件类型识别
- `order_book`：
  - 添加订单
  - 最优买卖价
  - 价差
  - 深度
  - 取消订单
  - 订单索引查询
  - 基础撮合
- `engine`：
  - 订单请求进入订单簿
  - 撮合后输出 `Trade`
  - 撮合后输出 `OrderUpdate`
  - `process_market_tick()` 串联策略与引擎
- `strategy`：
  - 阈值买入
  - 阈值卖出
  - 区间内不交易
  - 忽略其他 symbol

测试组织：

```text
src/order_book/tests.rs
src/engine/tests.rs
```

---

## Roadmap

已完成：

- [x] Module 0：项目蓝图
- [x] Module 1：Rust 基础模型与语法闭环
- [x] Module 2：双边订单簿与基础撮合
- [x] Module 3：策略接口与事件驱动模型

下一阶段：

- [ ] Module 4：Tokio 异步任务与消息通道
- [ ] 将同步事件流改造成异步 event loop
- [ ] 使用 channel 连接 market data、strategy、engine
- [ ] 引入回测事件循环
- [ ] 增加更完整的撮合规则、手续费、滑点
- [ ] 生成回测报告
- [ ] 接入 tracing 日志
- [ ] 接入 Axum API
- [ ] 接入 metrics / storage

---

## Project Positioning

这个项目用于展示 Rust 在交易系统类工程中的实践能力：

- 类型驱动的领域建模
- 明确的状态流转
- 可测试的订单簿和撮合逻辑
- 从同步事件流逐步演进到异步架构
- 从小模块开始，持续推进到策略、回测、API、指标和存储

当前它已经不是单独的练习代码，而是一个可以继续扩展的交易系统 mini 版。

