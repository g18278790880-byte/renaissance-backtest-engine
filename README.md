# Renaissance Backtest Engine

> 一个基于 Rust 的事件驱动交易回测与订单管理引擎，从核心交易模型开始逐步构建。

Renaissance Backtest Engine 是一个面向交易系统工程实践的 Rust 项目。它从行情、订单、成交、订单簿等基础模型出发，逐步扩展为一个事件驱动的回测引擎，覆盖策略执行、撮合模拟、持仓统计、API、日志、指标和性能测试等能力。

当前项目处于订单簿模块的早期阶段：已经定义了第一批核心市场与订单模型，支持基础订单状态流转，能够用价格索引保存买盘订单，并查询当前最高买价。

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
- 初版 `OrderBook`：
  - 使用 `BTreeMap<i64, Vec<Order>>` 按价格档位保存买盘订单；
  - 支持 `add_order()` 添加订单；
  - 支持 `best_bid()` 查询当前最高买价；
- 单元测试覆盖订单取消规则和最高买价查询。

当前里程碑：

```text
Module 2：订单簿
状态：进行中
最新完成：best_bid() - 查询当前最高买价
```

---

## 已实现示例

当前可执行程序会构建一个简单的买盘订单簿，并输出最高买价：

```rust
let mut order_book = OrderBook::new();

order_book.add_order(Order {
    id: 1,
    symbol: String::from("BTCUSDT"),
    side: Side::Buy,
    price: 99_000,
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

assert_eq!(order_book.best_bid(), Some(100_000));
```

`best_bid()` 利用 `BTreeMap` 的有序 key，取最后一个价格档位：

```rust
pub fn best_bid(&self) -> Option<i64> {
    self.bids.keys().next_back().copied()
}
```

当订单簿为空时，函数返回 `None`，避免使用特殊价格作为哨兵值。

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
| 买盘订单簿 | 进行中 | 已实现 `add_order()` 和 `best_bid()` |
| 卖盘订单簿 | 未开始 | 后续添加 `asks` 和 `best_ask()` |
| 订单簿内取消订单 | 未开始 | 后续支持从订单簿中移除或更新订单 |
| 价差与深度查询 | 未开始 | 后续实现 `spread()` 和价格档位深度 |
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
- 插入两笔买单；
- 输出当前最高买价；
- 输出订单簿的调试信息。

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

订单簿模块接下来会继续补齐：

1. 添加卖盘存储 `asks`；
2. 实现 `best_ask()`；
3. 计算买卖价差 `spread()`；
4. 支持在订单簿中取消或移除订单；
5. 提供按价格档位查询深度的能力。

订单簿稳定后，项目会进入事件模型和策略执行模块。

---

## 项目定位

这个仓库用于展示 Rust 在交易系统类问题中的工程实践：

- 类型驱动的领域建模；
- 明确、可测试的状态流转；
- 逐步构建的市场微观结构组件；
- 从小模块验证开始，逐渐扩展到异步服务、API、指标和存储。

