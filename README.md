# Renaissance Backtest Engine

> 一个基于 Rust 的事件驱动交易回测与订单管理引擎，从核心交易模型开始逐步构建。

Renaissance Backtest Engine 是一个面向交易系统工程实践的 Rust 项目。它从行情、订单、成交、订单簿等基础模型出发，逐步扩展为一个事件驱动的回测引擎，覆盖策略执行、撮合模拟、持仓统计、API、日志、指标和性能测试等能力。

当前项目已经从单独的订单簿模块推进到事件驱动雏形：系统可以接收 `Event::OrderRequest`，由 `Engine` 转换为订单并写入 `OrderBook`，在买卖价格交叉时完成基础撮合，并返回 `Event::Trade` 和 `Event::OrderUpdate`。策略侧也已开始实现，当前有 `Strategy` trait 和一个简单的 `ThresholdStrategy`。

---

## 项目目标

本项目的长期目标是构建一个迷你版交易基础设施：能够回放行情数据、更新市场状态、驱动策略决策、管理订单、模拟成交，并输出回测结果。

```text
market-data-simulator
        ↓
event-bus / channel
        ↓
engine
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

- 使用 Rust 类型系统建模 `Tick`、`OrderRequest`、`Order`、`OrderUpdate`、`Trade` 和订单状态；
- 维护可预测、可测试的价格档位订单簿；
- 通过 `Event` 和 `Engine` 把订单请求、订单簿、订单状态变化和成交事件连接起来；
- 通过 `Strategy` trait 将行情 tick 转换为订单请求；
- 用单元测试验证订单生命周期、事件处理、策略输出、订单簿行为和基础撮合；
- 在核心逻辑稳定后补充策略接口、回测循环、API、日志、指标、存储和性能测试。

---

## 当前进展

已经实现：

- Rust 项目骨架，包含 `main.rs`、`model.rs`、`event.rs`、`engine.rs`、`strategy.rs` 和 `order_book.rs`；
- 核心数据模型：
  - `Tick`
  - `OrderRequest`
  - `Order`
  - `OrderUpdate`
  - `Trade`
  - `Side`
  - `OrderStatus`
  - `OrderError`
- 事件模型：
  - `Event::MarketTick`
  - `Event::OrderRequest`
  - `Event::OrderUpdate`
  - `Event::Trade`
  - `Event::event_type()`
- 订单生命周期逻辑：
  - `Order::fill()`
  - `Order::cancel()`
  - 对已成交、已取消、已拒绝订单的取消校验；
- 初版双边 `OrderBook`：
  - 使用 `BTreeMap<i64, Vec<Order>>` 按价格档位分别保存买盘和卖盘订单；
  - 使用 `HashMap<u64, OrderLocation>` 维护订单 ID 到盘口位置的索引；
  - 支持 `add_order()` 添加订单，并拒绝重复订单 ID；
  - 支持 `cancel_order()` 从订单簿中取消并移除订单；
  - 支持 `best_bid()` 和 `best_ask()` 查询最优买卖价；
  - 支持 `best_bid_order()` 和 `best_ask_order()` 查询最优价格上的第一笔订单；
  - 支持 `spread()` 计算买卖价差；
  - 支持 `bid_depth()` 和 `ask_depth()` 聚合盘口深度；
  - 支持 `match_best_orders()` 撮合一组最优买卖订单；
  - 支持 `match_orders()` 连续撮合，直到价格不再交叉；
- 初版 `Engine`：
  - 接收 `Event` 作为输入；
  - 处理 `Event::OrderRequest`；
  - 将 `OrderRequest` 转换为带内部 ID 的 `Order`；
  - 调用 `OrderBook` 执行添加订单与撮合；
  - 将撮合结果包装为 `Event::Trade` 返回；
  - 根据撮合后的订单状态生成 `Event::OrderUpdate`；
  - 暴露 `order_count()`、`best_bid()`、`best_ask()` 便于观察当前状态；
- 初版策略接口：
  - 定义 `Strategy` trait；
  - 支持 `on_tick(&Tick) -> Vec<OrderRequest>`；
  - 预留 `on_order_update(&OrderUpdate)` 回调；
  - 实现 `ThresholdStrategy`，当价格低于买入阈值时生成买单，高于卖出阈值时生成卖单；
- 测试代码已从 `order_book.rs` 拆分到 `src/order_book/tests.rs`，并按行为整理为添加订单、盘口查询、取消订单、索引查询和撮合测试；
- 已修复非 `dead_code` 类 warning：包括 Clippy 枚举命名提示，以及测试中未处理 `Result` 的问题；
- 单元测试覆盖订单取消规则、事件类型、引擎事件处理、策略输出、最优买卖价、价差、盘口深度、订单索引和基础撮合。

当前里程碑：

```text
Module 3：策略接口与事件驱动模型
状态：事件引擎已能输出 Trade / OrderUpdate，策略接口已开始
最新完成：OrderUpdate Event 输出 / Strategy trait / ThresholdStrategy
```

---

## 已实现示例

当前引擎可以处理订单请求事件，并在价格交叉时返回成交事件和订单状态变化事件：

```rust
let mut engine = Engine::new();

let buy_event = Event::OrderRequest(OrderRequest {
    symbol: String::from("BTCUSDT"),
    side: Side::Buy,
    price: 100_000,
    quantity: 2,
});

let sell_event = Event::OrderRequest(OrderRequest {
    symbol: String::from("BTCUSDT"),
    side: Side::Sell,
    price: 99_000,
    quantity: 1,
});

let output_events = engine.handle_event(buy_event).unwrap();
assert!(output_events.is_empty());

let output_events = engine.handle_event(sell_event).unwrap();
assert_eq!(output_events.len(), 3);
```

当前系统主链路：

```text
Event
  ↓
Engine
  ↓
OrderBook
  ↓
Trade Event + OrderUpdate Event
```

策略侧当前已经具备从 tick 生成订单请求的能力：

```text
Tick
  ↓
Strategy
  ↓
OrderRequest
```

示例策略：

```rust
let mut strategy = ThresholdStrategy::new(String::from("BTCUSDT"), 99_000, 101_000, 1);

let tick = Tick {
    symbol: String::from("BTCUSDT"),
    price: 98_000,
    quantity: 1,
    timestamp: 1_717_000_000,
};

let requests = strategy.on_tick(&tick);
assert_eq!(requests.len(), 1);
```

`Engine` 目前负责把外部订单请求事件转为内部订单，并调用订单簿完成撮合：

```rust
pub fn handle_event(&mut self, event: Event) -> Result<Vec<Event>, EngineError> {
    match event {
        Event::MarketTick(_tick) => Ok(Vec::new()),
        Event::OrderRequest(request) => self.handle_order_request(request),
        Event::OrderUpdate(_update) => Ok(Vec::new()),
        Event::Trade(_trade) => Ok(Vec::new()),
    }
}
```

撮合逻辑当前采用简化版本：当最高买价大于或等于最低卖价时，以卖方价格生成成交，并根据成交数量更新订单状态；完全成交的订单会从订单簿和订单索引中移除。引擎会把成交包装为 `Event::Trade`，并为买卖双方生成对应的 `Event::OrderUpdate`。

---

## 架构路线

当前已实现的模块结构：

```text
src/
├── main.rs              # 可执行入口与当前事件流演示
├── model.rs             # Tick / OrderRequest / Order / OrderUpdate / Trade 等核心模型
├── event.rs             # Event 枚举，连接行情、订单请求、订单更新和成交
├── engine.rs            # 事件处理引擎：Event → OrderBook → output Events
├── strategy.rs          # Strategy trait 与 ThresholdStrategy
├── order_book.rs        # 价格档位订单簿
└── order_book/
    └── tests.rs         # 订单簿测试，按行为分组
```

当前订单簿测试结构：

```text
src/order_book/
└── tests.rs
    ├── add_order_tests
    ├── quote_tests
    ├── cancel_tests
    ├── lookup_tests
    └── matching_tests
```

后续计划中的模块：

```text
src/
├── execution.rs         # 更完整的撮合、手续费、滑点模拟
├── backtest.rs          # 事件回放循环与回测报告生成
├── portfolio.rs         # 持仓、现金、PnL 统计
├── api.rs               # Axum HTTP API
├── metrics.rs           # 运行指标与回测指标
└── storage.rs           # SQLite / SQLx 持久化
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
| 核心模型 | 已完成 | `Tick`、`OrderRequest`、`Order`、`OrderUpdate`、`Trade`、方向、状态、错误枚举 |
| 事件模型 | 已完成 | 已实现 `Event` 枚举和 `event_type()` |
| 引擎雏形 | 已完成 | 已实现 `Engine::handle_event()` 处理订单请求并返回成交与订单更新事件 |
| 策略接口 | 已开始 | 已实现 `Strategy` trait 和 `ThresholdStrategy` |
| 订单生命周期 | 已完成 | 已实现成交和取消逻辑，并配有单元测试 |
| 双边订单簿 | 已完成 | 已实现买盘 `bids` 和卖盘 `asks` |
| 最优价查询 | 已完成 | 已实现 `best_bid()` 和 `best_ask()` |
| 价差查询 | 已完成 | 已实现 `spread()` |
| 盘口深度查询 | 已完成 | 已实现 `bid_depth()` 和 `ask_depth()` |
| 订单簿内取消订单 | 已完成 | 已实现 `cancel_order()` 和订单索引维护 |
| 最优订单查询 | 已完成 | 已实现 `best_bid_order()` 和 `best_ask_order()` |
| 基础撮合 | 已完成 | 已实现 `match_best_orders()` 和 `match_orders()` |
| Trade Event 输出 | 已完成 | 引擎可将撮合结果返回为 `Event::Trade` |
| OrderUpdate Event 输出 | 已完成 | 引擎可为成交双方返回 `Event::OrderUpdate` |
| 测试拆分与整理 | 已完成 | 订单簿测试已拆到 `src/order_book/tests.rs` 并按行为分组 |
| Warning 整理 | 已完成 | 已修复非 `dead_code` 类 warning |
| Tick → Strategy → OrderRequest | 已完成 | 策略可基于行情阈值生成订单请求 |
| Strategy → Engine 串联 | 下一步 | 将策略输出的订单请求送入引擎处理 |
| 回测事件循环 | 未开始 | 后续连接行情、策略、订单和成交事件 |
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

- 创建一个 `ThresholdStrategy`；
- 输入一条 `Tick`；
- 根据价格阈值生成 `OrderRequest`；
- 输出策略生成的订单请求。

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

当前系统已经形成两段链路：

```text
OrderRequest → Engine → OrderBook → Trade / OrderUpdate

Tick → Strategy → OrderRequest
```

下一步重点是把这两段串起来，形成同步版最小业务闭环：

```text
MarketTick
  ↓
Strategy.on_tick()
  ↓
OrderRequest
  ↓
Engine.handle_event()
  ↓
OrderBook
  ↓
Trade / OrderUpdate
```

下一步可实现一个同步事件处理函数，例如 `process_market_tick()` 或 `run_tick()`：

1. 接收一条 `Tick`；
2. 调用策略生成 `OrderRequest`；
3. 将订单请求包装成 `Event::OrderRequest`；
4. 交给 `Engine::handle_event()`；
5. 收集并返回 `Trade` / `OrderUpdate` 等输出事件。

完成这一步后，项目就会从“订单簿 + 策略零件”升级成一个同步版事件驱动交易系统 mini 版，然后再进入 Tokio/channel 的异步化阶段。

---

## 项目定位

这个仓库用于展示 Rust 在交易系统类问题中的工程实践：

- 类型驱动的领域建模；
- 明确、可测试的状态流转；
- 从订单簿推进到事件驱动引擎；
- 逐步构建的市场微观结构组件；
- 从小模块验证开始，逐渐扩展到策略、回测、异步服务、API、指标和存储。
