# Solana gRPC 事件监听 & 扩展框架

这个仓库提供一个面向 Pump / PumpAmm 程序的 Solana gRPC 事件监听骨架，已经将 `create_v2` 与 Mayhem 模式纳入解析流程，方便后续扩展交易、风控、策略等功能。

## 支持的事件

### Pump 程序 `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P`
- `CreateEvent / CreateV2Event`：创建代币（含 `is_mayhem_mode` 标记，来自 [Pump 公告](https://github.com/pump-fun/pump-public-docs/blob/main/README.md) 中的 Token2022 / Mayhem 更新）
- `CompleteEvent`：曲线完成
- `TradeEvent`：买卖撮合

### PumpAmm 程序 `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA`
- `BuyEvent`：买入
- `SellEvent`：卖出
- `CreatePoolEvent`：创建池

## 功能特点

- 基于 `yellowstone-grpc`，可同时订阅多个程序
- 事件解析统一抽象成 `EventTrait`，新增事件时只需补充模型与 discriminator
- `CreateV2Event` 与 `is_mayhem_mode` 已对接，便于扩展 Token2022 相关策略
- 支持在 `src/client/grpc.rs` 中填充自定义处理逻辑（目前默认输出事件日志）

## 快速开始

1. 准备环境  
   ```bash
   git clone https://github.com/Fuck-Meme/grpc.git
   cd grpc
   cp .env.example .env   # 如需
   ```
2. 设置环境变量  
   ```env
   YELLOWSTONE_GRPC_URL="https://solana-yellowstone-grpc.publicnode.com"
   ```
3. 构建并运行  
   ```bash
   cargo build --release
   cargo run
   ```

程序启动后会订阅 Pump 与 PumpAmm 两个程序，并打印解析后的事件。  

## 目录结构

```
src/
├── client/         # gRPC 客户端及事件处理入口
├── models/         # Borsh 事件模型（含 CreateV2Event）
├── parser/         # discriminator 解析与访问器
└── main.rs         # 程序入口，初始化依赖并启动订阅
```

## 后续扩展建议

- **策略/交易执行**：在 `handle_logs` 中换成发送消息、落库或下单逻辑
- **Mayhem 模式处理**：根据 `is_mayhem_mode` 分流到不同的费率配置或风控通道
- **多程序支持**：扩展 `program_ids` 或引入配置文件动态加载

## 依赖

- `tokio`
- `yellowstone-grpc-client`
- `borsh`
- `solana-sdk`
- `dotenvy`
- `log`

## 参考

- Pump 官方文档：[pump-public-docs](https://github.com/pump-fun/pump-public-docs/blob/main/README.md)

## 许可证

MIT

