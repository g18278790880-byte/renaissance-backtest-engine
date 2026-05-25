# Renaissance Backtest Engine

> 基于 Rust/Tokio 的事件驱动交易回测与订单管理系统。

本项目不是为了“泛学 Rust”，而是为了用 Rust 构建一个迷你版交易系统：能够接收模拟行情、维护订单簿、运行策略、生成订单、模拟成交、执行回测，并逐步补充 API、日志、指标和性能测试。

---

## 1. 项目背景

量化交易系统的核心问题是：当行情数据持续进入系统后，系统如何快速、稳定、可观测地完成以下流程：

1. 接收行情事件；
2. 更新订单簿或市场状态；
3. 驱动策略生成订单；
4. 经过订单管理与撮合模拟；
5. 更新持仓、成交和收益；
6. 输出回测报告、日志与性能指标。

Rust 适合这个项目的原因在于：它强调内存安全、类型建模、零成本抽象和并发可靠性，适合构建性能敏感、可靠性要求高的交易基础设施。

---

## 2. 项目目标

本项目最终目标是实现一个事件驱动的交易系统 mini 版：

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
backtest-report + API + metrics
```

最小可运行目标：

- 从 CSV 或模拟行情源读取 tick 数据；
- 定义 `Tick`、`Order`、`Trade`、`Position` 等核心模型；
- 维护买卖盘订单簿；
- 实现一个简单策略接口；
- 运行事件驱动回测；
- 输出基础回测报告；
- 使用单元测试验证核心逻辑。

增强目标：

- 使用 Tokio 组织异步任务；
- 使用 Axum 暴露 HTTP API；
- 使用 tracing 输出结构化日志；
- 使用 Criterion 做性能基准测试；
- 使用 SQLite/SQLx 保存订单、成交和回测结果；
- 输出 Prometheus 风格的 `/metrics` 指标。

---

## 3. 岗位匹配点

| 岗位要求 | 项目对应能力 |
| --- | --- |
| 基于 Rust 的高性能、低时延算法交易系统 | Rust 数据建模、订单簿、事件循环、Benchmark |
| 数据处理平台、监控运维平台 | CSV/Polars 数据处理、tracing 日志、metrics 指标 |
| 面向客户的高可用交易工具 | Axum API、状态查询、回测任务触发 |
| 策略相关回测平台 | 策略 trait、事件回放、撮合模拟、回测报告 |

---

## 4. 系统架构

```text
+-----------------------+
| Market Data Simulator |
| CSV / Mock Tick Feed  |
+----------+------------+
           |
           v
+-----------------------+
| Event Bus             |
| Tokio mpsc / channel  |
+----------+------------+
           |
           v
+-----------------------+
| Strategy Engine       |
| Strategy Trait        |
+----------+------------+
           |
           v
+-----------------------+
| Order Manager         |
| Order Request/Update  |
+----------+------------+
           |
           v
+-----------------------+
| Execution Simulator   |
| Fill / Fee / Slippage |
+----------+------------+
           |
           v
+-----------------------+
| Portfolio & Report    |
| PnL / Position / JSON |
+-----------------------+
```

后续 API 层：

```text
Axum HTTP API
├── GET  /health
├── GET  /orders
├── GET  /positions
├── POST /backtests
├── GET  /backtests/{id}
└── GET  /metrics
```

---

## 5. 核心模块规划

```text
src/
├── main.rs          # 程序入口
├── model.rs         # Tick / Order / Trade / Position 等核心数据结构
├── event.rs         # Event 枚举，连接行情、策略、订单和成交
├── order_book.rs    # 订单簿：add/cancel/best_bid/best_ask/depth
├── strategy.rs      # 策略 trait 与示例策略
├── execution.rs     # 撮合与成交模拟
├── backtest.rs      # 回测事件循环与报告生成
├── portfolio.rs     # 持仓、现金、PnL 统计
├── api.rs           # Axum API，后续实现
├── metrics.rs       # 指标统计，后续实现
└── storage.rs       # SQLx/SQLite 存储，后续实现
```

当前 Module 0 只创建项目骨架，不急着实现所有模块。

---

## 6. 技术栈

| 类型 | 技术 |
| --- | --- |
| 核心语言 | Rust |
| 异步运行时 | Tokio |
| API 服务 | Axum |
| 序列化 | Serde / serde_json |
| 数据处理 | CSV / Polars |
| 存储 | SQLite / SQLx |
| 日志 | tracing / tracing-subscriber |
| 性能测试 | Criterion |
| 测试 | Rust built-in test framework |

---

## 7. 当前进度

- [x] 创建 Rust 项目
- [x] 编写 README 项目蓝图
- [ ] 定义核心数据结构
- [ ] 实现订单簿
- [ ] 实现策略接口
- [ ] 实现事件循环
- [ ] 实现行情模拟器
- [ ] 实现回测引擎
- [ ] 接入 Axum API
- [ ] 接入 tracing 日志
- [ ] 添加 Criterion benchmark

---

## 8. Module 0 任务清单

Module 0 的目标不是写复杂代码，而是回答清楚一个问题：

> 这个项目到底要证明我具备什么能力？

本模块交付物：

- [x] 项目名称；
- [x] 项目目标；
- [x] 系统架构；
- [x] 核心模块；
- [x] 技术栈；
- [x] 待实现功能。

完成 Module 0 后，进入 Module 1：Rust 最小语法闭环。

---

## 9. 如何运行

当前项目还没有业务逻辑，只用于验证 Rust 工程是否能正常启动。

```bash
cargo run
```

预期输出：

```text
rust-trading-backtest-engine: Module 0 initialized
```

运行测试：

```bash
cargo test
```

格式化代码：

```bash
cargo fmt
```

检查代码：

```bash
cargo check
```

---

## 10. 后续开发顺序

建议按照以下顺序推进：

1. Module 1：Rust 最小语法闭环  
   先实现 `Tick`、`Order`、`Side`、`Trade` 等基础模型。

2. Module 2：订单簿  
   实现 `add_order`、`cancel_order`、`best_bid`、`best_ask`、`spread`。

3. Module 3：策略接口与事件模型  
   使用 trait 和 enum 解耦行情、策略、订单和成交。

4. Module 4：Tokio 异步消息通道  
   用 channel 串起行情、策略和执行模块。

5. Module 5-6：行情模拟器与回测引擎  
   从 CSV 读取 tick，事件回放，生成回测报告。

6. Module 7-10：API、存储、日志和性能测试  
   让项目从“能跑”升级为“像真实工程”。

---

## 11. 简历表达草稿

项目名称：

> 基于 Rust/Tokio 的事件驱动交易回测与订单管理系统

项目描述：

> 面向量化交易系统开发场景，使用 Rust 构建事件驱动的交易回测与订单管理系统，支持行情事件回放、策略信号生成、订单管理、撮合模拟、持仓统计和回测报告输出；后续将接入 Tokio 异步任务、Axum API、tracing 日志和 Criterion 性能测试，用于验证系统在低延迟、高可靠交易基础设施中的工程能力。

---

## 12. 学习纪律

本项目遵循 LDLT 循环：

```text
Learn  学一点
Do     立刻写一点
Learn  卡住后再补一点
Teach  用 README / 注释 / 总结讲出来
```

关键规则：

- 不先通读全书；
- 不追求一次性设计完美；
- 每个模块必须有可运行代码；
- 每个核心模块必须写测试；
- 每次完成一个功能，都更新 README；
- 遇到编译错误，要解释清楚，而不是绕过去。
