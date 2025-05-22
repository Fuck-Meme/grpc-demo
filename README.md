# Solana 智能合约事件监控

这是一个用于监控 Solana 区块链上特定智能合约事件的 Rust 项目。项目使用 gRPC 订阅 Solana 的交易日志，并解析特定合约的事件。

## 功能特性

- 使用 gRPC 实时订阅 Solana 交易
- 支持监控多个智能合约
- 解析并处理以下事件：
  - Pump 合约事件：
    - 创建代币 (Create)
    - 曲线完成 (Complete)
    - 交易 (Trade)
  - PumpAmm 合约事件：
    - 买入 (Buy)
    - 卖出 (Sell)
    - 创建池 (CreatePool)
- 事件数据封装，方便在其他地方使用

## 项目结构

```
src/
├── client/         # gRPC 客户端实现
├── handle/         # 事件处理逻辑
├── models/         # 事件数据模型
├── parser/         # 日志解析器
└── main.rs         # 程序入口
```

## 使用方法

1. 克隆项目
```bash
git clone https://github.com/Fuck-Meme/grpc-demo.git
cd grpc-demo
```

2. 运行项目
```bash
cargo run
```

3. 在代码中使用事件处理
```rust
use grpcdemo::handle::EventHandler;

let handler = EventHandler::new();

// 解析 Pump 事件
let pump_events = handler.parse_pump_events(&logs);
if let Some(trade) = pump_events.trade {
    // 处理 trade 事件
}

// 解析 PumpAmm 事件
let pump_amm_events = handler.parse_pump_amm_events(&logs);
if let Some(buy) = pump_amm_events.buy {
    // 处理 buy 事件
}
```

## 依赖

- Rust 1.85.0 或更高版本
- yellowstone-grpc-client
- tokio
- log

## 输出格式

事件输出格式如下：

```
-----------------------------------------------
slot: 341680670
tx: 4r9S2mpUSCCXuNbpUTKrrdyKxuTetkpz1CaXb3u3JLsFMHbHzr24WuDVaGdtbBf4hKZZuVkhsovRD4R1dKpCYjyB
events:
  - TradeEvent { mint: 145sZrmo2nwyZE5WssJW4onD1zBvuRTZBbREKR1Gpump, ... }
-----------------------------------------------
```

## 许可证

MIT 