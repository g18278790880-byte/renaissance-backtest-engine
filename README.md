# Renaissance Backtest Engine

一个使用 Rust 构建的事件驱动交易回测与订单管理系统 mini 版。

当前项目已经从同步核心链路推进到 Tokio 异步消息通道版本：行情 tick 通过 channel 进入策略任务，策略生成订单请求，执行任务将订单写入内存订单簿，价格交叉时生成成交和订单状态更新，并由事件日志任务消费输出事件。

## Overview

Renaissance Backtest Engine 是一个面向交易基础设施建模的 Rust 项目。当前已经实现：

- 基础交易数据模型，例如 tick、订单、成交、订单更新；
- 基于买卖盘价格档位的内存限价订单簿；
- 简化版 price-time priority 撮合逻辑；
- 策略接口和一个阈值策略示例；
- 同步事件驱动引擎，将策略输出、订单簿状态和执行事件串联起来；
- 基于 Tokio `mpsc` channel 的异步 task 流水线。

项目目前保持较小的实现范围，优先验证核心状态流转、订单簿行为、异步任务边界和测试闭环。CSV 行情回放、回测报告、API、日志、性能测试和持久化等能力仍处于规划阶段。

## Why This Project

这个项目的目标不是泛泛学习 Rust 语法，而是用 Rust 逐步构建一个性能敏感、状态严谨、事件驱动的交易系统基础设施。

它对应量化交易系统中的常见模块：

| 交易系统领域 | 当前或计划中的项目模块 |
| --- | --- |
| 行情数据 | `Tick` 模型，内存 demo 行情模拟器，后续计划加入 CSV 回放 |
| 策略接口 | `Strategy` trait 和 `ThresholdStrategy` |
| 订单管理 | `OrderRequest`、`Order`、`OrderStatus`、`OrderUpdate` |
| 执行模拟 | `OrderBook` 撮合和 `Trade` 事件 |
| 事件驱动架构 | `Event` enum 和 `Engine` 处理流程 |
| 异步消息通道 | Tokio task 和 `mpsc` channel |
| 回测平台 | 后续计划加入历史回放、持仓、PnL 和报告层 |

Rust 适合这类项目，因为它能够用类型系统明确表达状态、所有权和错误路径，同时避免垃圾回收带来的运行时不确定性。

## Current Status

| 模块 | 状态 | 说明 |
| --- | --- | --- |
| 交易数据模型 | 已实现 | `Tick`、`OrderRequest`、`Order`、`OrderUpdate`、`Trade`、`Side`、`OrderStatus` |
| 事件模型 | 已实现 | `Event::{MarketTick, OrderRequest, OrderUpdate, Trade}` |
| 订单簿存储 | 已实现 | 买盘和卖盘使用 `BTreeMap<i64, Vec<Order>>` |
| 订单 ID 索引 | 已实现 | 使用 `HashMap<u64, OrderLocation>` |
| best bid / best ask | 已实现 | 查询最高买价和最低卖价 |
| spread | 已实现 | 双边存在时返回 `best_ask - best_bid` |
| depth | 已实现 | 按价格档位聚合数量和订单数 |
| add order | 已实现 | 支持重复订单 ID 拒绝 |
| cancel order | 已实现 | 从订单簿和索引中移除活跃订单 |
| order lookup | 已实现 | `contains_order` 和 `get_order` |
| matching | 已实现 | 支持价格交叉、部分成交、完全成交和连续撮合 |
| Strategy trait | 已实现 | 包含 `on_tick` 和 `on_order_update` hook |
| ThresholdStrategy | 已实现 | 用于测试系统链路的简单阈值策略 |
| DemoCrossStrategy | 已实现 | 用于异步流水线测试的交叉订单策略 |
| Engine | 已实现 | 连接策略请求、订单簿写入、撮合和输出事件 |
| MarketDataSimulator | 已实现 | 支持内存 tick 列表和 demo crossed ticks |
| Tokio 异步任务 | 已实现 | `market_data_task`、`strategy_task`、`execution_task`、`event_logger_task` |
| Channel 流水线 | 已实现 | tick、order request、output event 通过 `mpsc` channel 传递 |
| 单元测试 | 已实现 | 当前 45 个测试通过 |
| 回测引擎 | 计划中 | 尚未实现 |
| HTTP API | 计划中 | 尚未实现 |
| 持久化 | 计划中 | 尚未实现 |
| benchmark | 计划中 | 尚未实现 |

当前二进制运行时会出现少量 Rust dead-code warning。原因是一些状态和接口已经提前建模，或仅在测试中使用，但尚未全部接入 `main` 示例流程。当前阶段的重点是核心建模、撮合逻辑和测试闭环。

## Architecture

```mermaid
flowchart LR
    Simulator[MarketDataSimulator] --> TickChannel[tick channel]
    TickChannel --> StrategyTask[strategy_task]
    StrategyTask --> OrderChannel[order request channel]
    OrderChannel --> ExecutionTask[execution_task]
    ExecutionTask --> Engine[Engine]
    Engine --> Book[OrderBook]
    Engine --> EventChannel[event channel]
    EventChannel --> Logger[event_logger_task]
```

当前异步流程：

1. `market_data_task` 使用 `MarketDataSimulator` 发送 demo ticks。
2. tick 通过 Tokio `mpsc` channel 进入 `strategy_task`。
3. `strategy_task` 调用 `Strategy::on_tick`，生成零个或多个 `OrderRequest`。
4. order request 通过 channel 进入 `execution_task`。
5. `execution_task` 使用 `Engine` 写入订单簿并触发撮合。
6. 撮合结果生成 `Trade` 和 `OrderUpdate` 事件。
7. output events 通过 channel 发送给 `event_logger_task`。

同步的 `Engine::process_market_tick` 仍然保留，用于测试和较小范围的直接调用；当前 `main` 已经使用 Tokio 异步任务和 channel 运行完整 demo 流程。

## Core Concepts

| 概念 | 职责 |
| --- | --- |
| `Tick` | 行情输入，包含 symbol、price、quantity 和 timestamp |
| `OrderRequest` | 策略生成的订单意图，尚未分配订单 ID |
| `Order` | 进入订单簿后的活跃订单 |
| `OrderBook` | 内存买卖盘、订单索引和撮合逻辑 |
| `Trade` | 买卖订单成交后生成的执行结果 |
| `OrderUpdate` | 撮合后生成的订单状态和剩余数量更新 |
| `Event` | 系统内部事件枚举 |
| `Engine` | 协调策略输出、订单写入、撮合和事件输出 |
| `Strategy` | 将行情 tick 转换为订单请求的策略接口 |
| `MarketDataSimulator` | 从内存 tick 列表向 channel 推送行情 |
| `tasks` | Tokio 异步任务边界，连接行情、策略、执行和事件日志 |

## Project Structure

```text
.
├── Cargo.toml
├── README.md
└── src
    ├── main.rs
    ├── market_data.rs
    ├── model.rs
    ├── event.rs
    ├── strategy.rs
    ├── tasks.rs
    ├── engine.rs
    ├── engine
    │   └── tests.rs
    ├── order_book.rs
    └── order_book
        └── tests.rs
```

## Implemented Features

### Data Models

`src/model.rs` 定义交易系统的核心类型：

- `Side`：买卖方向；
- `OrderStatus`：新建、部分成交、完全成交、已取消、已拒绝；
- `Tick`：行情输入；
- `OrderRequest`：策略生成的订单请求；
- `Order`：订单簿内部订单；
- `OrderUpdate`：订单状态更新；
- `Trade`：买卖订单撮合后的成交记录。

`OrderRequest` 和 `Order` 被刻意分离。策略只表达交易意图，不负责分配订单 ID；订单 ID 由引擎统一生成。

### Order Book

`src/order_book.rs` 实现内存订单簿：

- 买盘和卖盘使用 `BTreeMap<i64, Vec<Order>>` 存储；
- 买盘按价格从高到低读取；
- 卖盘按价格从低到高读取；
- 同一价格档位内用 `Vec<Order>` 保持插入顺序；
- 使用 `HashMap<u64, OrderLocation>` 支持订单 ID 查询；
- `DepthLevel` 聚合价格、总数量和订单数；
- 重复订单 ID 会被拒绝。

当前支持的查询和操作：

- `add_order`；
- `cancel_order`；
- `best_bid`；
- `best_ask`；
- `spread`；
- `bid_depth`；
- `ask_depth`；
- `order_count`；
- `contains_order`；
- `get_order`；
- `best_bid_order`；
- `best_ask_order`。

### Matching Engine

订单簿支持简化撮合：

- 当 `best_bid >= best_ask` 时发生撮合；
- 当前成交价取卖盘价格；
- 成交数量取买卖双方剩余数量的较小值；
- 完全成交的订单会从订单簿和订单 ID 索引中移除；
- 部分成交的订单保留在订单簿中，并减少剩余数量；
- `match_orders` 会持续撮合，直到最优买卖价不再交叉。

这是用于当前阶段建模和测试的简化 price-time priority 实现，并不试图完整复刻生产级交易所撮合引擎。

### Strategy

`src/strategy.rs` 定义：

- `Strategy` trait，包含 `on_tick` 和 `on_order_update` hook；
- `ThresholdStrategy`，一个最小阈值策略；
- `DemoCrossStrategy`，用于异步流水线测试的交叉订单策略。

`ThresholdStrategy` 用于测试系统链路：

- 当价格小于等于 `buy_below` 时生成买单；
- 当价格大于等于 `sell_above` 时生成卖单；
- 忽略其他 symbol 的 tick；
- 当价格位于阈值区间内时不生成订单。

它是系统测试策略，不代表真实投资策略。

`DemoCrossStrategy` 会在前两次 tick 中生成一笔买单和一笔交叉卖单，用于验证 Tokio task pipeline 能够产生成交和订单更新事件。

### Engine

`src/engine.rs` 连接事件、策略和订单簿：

- 通过 `handle_event` 处理 `Event::OrderRequest`；
- 分配递增订单 ID；
- 将订单写入订单簿；
- 重复撮合交叉订单；
- 输出 `Event::Trade` 和 `Event::OrderUpdate`；
- 暴露 `order_count`、`best_bid`、`best_ask` 等简单查询；
- 通过 `process_market_tick` 打通当前 tick -> strategy -> order book 流程。

### Async Tasks

`src/tasks.rs` 定义当前 Tokio 异步流水线：

- `market_data_task`：运行内存行情模拟器，将 `Tick` 发送到 tick channel；
- `strategy_task`：接收 tick，调用策略，并将 `OrderRequest` 发送到 order channel；
- `execution_task`：接收订单请求，调用 `Engine` 和 `OrderBook`，并发送输出事件；
- `event_logger_task`：接收并打印 `Trade` / `OrderUpdate` 等事件。

当前 channel 使用 Tokio `mpsc`，用于验证任务拆分和消息传递边界。尚未实现背压策略、错误恢复、graceful shutdown 信号或真实网络行情接入。

### Market Data Simulator

`src/market_data.rs` 定义 `MarketDataSimulator`：

- 支持从内存 `Vec<Tick>` 创建模拟器；
- 提供 `demo_cross_ticks`，生成两条可以驱动交叉成交的示例 tick；
- 通过 async `run` 方法将 tick 发送到 channel；
- 提供 `len` 和 `is_empty` 用于测试和状态检查。

当前模拟器只使用内存 tick 列表。CSV 加载、按 timestamp 排序、定时回放和真实时间节奏控制属于下一阶段 Module 5。

## Getting Started

环境要求：

- Rust stable toolchain；
- Cargo。

运行示例：

```bash
cargo run
```

当前示例会启动四个 Tokio task：行情、策略、执行和事件日志。demo 行情包含两条 `BTCUSDT` tick，`DemoCrossStrategy` 会生成一笔买单和一笔交叉卖单，因此运行结果会打印一笔成交和两条订单更新。核心输出类似：

```text
market data simulator: all ticks sent
strategy task: order request generated: OrderRequest { ... side: Buy, price: 100000, quantity: 2 }
strategy task: order request generated: OrderRequest { ... side: Sell, price: 99000, quantity: 1 }
event logger task: event received: Trade(...)
event logger task: event received: OrderUpdate(...)
event logger task: event received: OrderUpdate(...)
```

运行测试：

```bash
cargo test
```

检查格式：

```bash
cargo fmt --check
```

## Testing

当前共有 45 个单元测试，覆盖：

- 数据模型行为和订单请求转换；
- 事件类型识别；
- 订单添加和重复订单 ID 拒绝；
- best bid / best ask 查询；
- spread 计算；
- bid / ask depth 聚合；
- 同价格档位 FIFO 行为；
- 撤单；
- 订单数量和订单 ID 查询；
- 价格交叉撮合；
- 部分成交和完全成交；
- 连续撮合直到价格不再交叉；
- 阈值策略行为；
- demo crossed strategy 行为；
- Engine 从订单请求到成交和订单更新的事件流；
- market tick 通过策略驱动订单簿；
- 内存行情模拟器；
- Tokio channel 发送 tick；
- 异步 pipeline 从行情、策略、执行到事件输出的完整链路。

当前本地验证结果：

```text
cargo test        # 45 passed
cargo fmt --check # passed
```

## Example Flow

一个简化场景：

1. `MarketDataSimulator` 发送两条 `Tick`。
2. `strategy_task` 接收 tick，并通过 `DemoCrossStrategy` 生成订单请求。
3. `execution_task` 接收 `OrderRequest`。
4. 引擎分配订单 ID，并创建 `Order`。
5. 订单簿存储订单。
6. 如果最优买价和最优卖价交叉，订单簿生成 `Trade`。
7. `event_logger_task` 接收成交事件和对应的订单状态更新。

示例代码：

```rust
let (tick_tx, tick_rx) = mpsc::channel::<Tick>(100);
let (order_tx, order_rx) = mpsc::channel::<OrderRequest>(100);
let (event_tx, event_rx) = mpsc::channel::<Event>(100);

let market_data_handle = tokio::spawn(market_data_task(tick_tx));
let strategy_handle = tokio::spawn(strategy_task(tick_rx, order_tx));
let execution_handle = tokio::spawn(execution_task(order_rx, event_tx));
let event_logger_handle = tokio::spawn(event_logger_task(event_rx));
```

在当前 demo 中，第一条 tick 生成买单，第二条 tick 生成交叉卖单。执行任务处理第二笔订单后，订单簿产生成交和两条订单更新事件。

## Roadmap

以下内容是后续计划，不是当前已完成能力：

- Module 5：行情模拟器与消息协议；
- CSV tick 加载和历史行情回放；
- 按 timestamp 排序和回放 tick；
- 可配置的回放速度或定时推送；
- JSON 消息格式和序列化；
- 可选的 TCP / WebSocket 行情输入原型；
- 回测引擎；
- portfolio / position 账户状态；
- fee、slippage 和 PnL 报告；
- Axum HTTP API，用于 orders、positions、backtests 和 metrics；
- 使用 `tracing` 进行结构化日志；
- 使用 Criterion 对订单簿插入、撤单、查询和撮合做 benchmark；
- 使用 SQLx / SQLite 做持久化；
- 随着运行时链路扩展，逐步清理当前 dead-code warning。

## Learning Milestones

| 阶段 | 状态 | 项目体现 |
| --- | --- | --- |
| Module 0：项目蓝图 | 已完成 | README、系统边界和 Roadmap |
| Module 1：Rust 基础模型 | 已完成 | `model.rs`、枚举、结构体、Result / Option、单元测试 |
| Module 2：订单簿 | 已完成 | `OrderBook`、价格档位、订单索引、撤单、撮合 |
| Module 3：策略接口与事件模型 | 已完成 | `Strategy`、`Event`、`Engine` |
| Module 4：Tokio 异步与消息通道 | 已完成 | `tasks.rs`、Tokio `mpsc`、异步 pipeline 测试 |
| Module 5：行情模拟器与消息协议 | 下一步 | CSV loader、tick replay、JSON / 简单消息协议 |

## Design Notes

- 使用 Rust 的 enum 和 struct 明确表达交易状态和事件类型。
- 使用 `Event` enum 统一建模行情、订单请求、成交和订单更新。
- 分离 `OrderRequest` 和 `Order`，避免策略层承担订单 ID 和引擎状态职责。
- 分离 `Trade` 和 `OrderUpdate`，因为成交事件和订单状态变化虽然相关，但语义不同。
- 使用 `BTreeMap` 保证价格档位的确定性排序。
- 使用 `HashMap` 支持按订单 ID 快速定位订单。
- 当前先保留同步 `Engine` 核心，再用 Tokio task 和 channel 包装异步流水线。这样可以让订单簿和撮合逻辑保持可测试，同时逐步演进到更接近真实交易系统的消息驱动架构。

## License

License: MIT，以 `Cargo.toml` 中声明为准。

仓库根目录当前没有独立的 `LICENSE` 文件。
