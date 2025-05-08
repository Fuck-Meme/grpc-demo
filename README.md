# Solana gRPC 事件监听器

这是一个用于监听 Solana 区块链上特定程序事件的 Rust 应用程序。目前支持监听以下程序的事件：

1. Pump 程序 (`6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P`)
   - CreateEvent: 创建代币事件
   - CompleteEvent: 曲线完成事件
   - TradeEvent: 交易事件

2. PumpAmm 程序 (`pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA`)
   - BuyEvent: 买入事件
   - CreatePoolEvent: 创建池事件
   - SellEvent: 卖出事件

## 功能特点

- 同时监听多个 Solana 程序的事件
- 支持实时事件解析和处理
- 使用 yellowstone-grpc 进行高效的事件订阅
- 支持 Borsh 序列化的事件数据解析

## 安装

1. 确保已安装 Rust 和 Cargo
2. 克隆仓库：
   ```bash
   git clone https://github.com/Fuck-Meme/grpc.git
   cd grpc
   ```
3. 创建 `.env` 文件并设置必要的环境变量：
   ```env
   YELLOWSTONE_GRPC_URL="https://solana-yellowstone-grpc.publicnode.com"
   ```

## 使用方法

1. 编译项目：
   ```bash
   cargo build --release
   ```

2. 运行程序：
   ```bash
   cargo run
   ```

程序将开始监听两个程序的事件，并在控制台输出相关信息。

## 项目结构

```
src/
├── client/         # gRPC 客户端实现
├── models/         # 事件数据模型
├── parser/         # 事件解析器
└── main.rs         # 程序入口点
```

## 依赖项

- tokio: 异步运行时
- yellowstone-grpc-client: Solana gRPC 客户端
- borsh: 序列化/反序列化
- solana-sdk: Solana 基础功能
- dotenvy: 环境变量管理
- log: 日志记录

## 注意事项

- 确保有稳定的网络连接
- 需要有效的 yellowstone-grpc 访问权限
- 建议在生产环境中使用适当的日志级别

## 许可证

MIT 