# rust-exercise

## 本筆記來自

origin:
https://candle.dev/blog/javascript-to-rust/javascript-to-rust-day-24-crates-and-tools/

translate:
https://zhuanlan.zhihu.com/p/455056963

github:
https://github.com/wasmflow/node-to-rust

## 安裝 rust 環境

安裝環境

```shell
# in macOS
brew install rustup-init
rustup-init
```

安裝必備全域套件

```shell
# 區域套件管理器
cargo install cargo-edit
# 指令行執行器
cargo install just
```

vscode 插件

- rust-analyzer
  - setting.json 添加
    "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
    },
    "rust-analyzer.checkOnSave.command": "clippy"
- rust
- Rust syntax

## 常見工具箱(Crates)

### 数据

- serde - 数据序列化生态系统。
- serde_json - JSON 解析和字符串化。
- parking_lot - 比 Rust 的 Mutex 和 RwLock 更好用。
- once_cell - 当需要全局变量时用到。

### 错误处理

- thiserror - 用于简单的自定义错误处理。
- anyhow - 用于简单的通用错误处理。

### 日志

- log - 日志接口，将日志的使用场景，从日志工具(logger)的具体实现中抽象出来。
- env_logger - 由环境变量控制的简易控制台日志。
- test-log - 在测试中获得 env_logger 风格的输出，而无需初始化一个 logger。
- pretty-env-logger - env_logg, 但是更美观.
- tracing - 是 log 和 env_logger 的直接替代品，具有扩展功能和一流的异步支持。

### 命令行界面(CLI)

- structopt - 用于从配置中获取 CLI 参数。
- clap - 用于从代码中获取 CLI 参数。
  异步/同步
- tokio - 异步运行时.
- tokio-stream - Tokio 的流处理工具。
- async-trait - 用于当实现有异步方法的特质时。
- crossbeam - 双向通信通信等等。
  网络
- rocket - 一个具有良好开发者体验的 HTTP 服务器。
- reqwest - 一个简单易用的 HTTP 客户端。
- hyper - 一个快速且正确的 HTTP 实现。
  期望在标准库中实现的功能
- rand - 随机数生成和相关工具。
- regex - 正则表达式。
- base64 - Base64 实现。
- http - 一个常见的 HTTP 类型通用库。

### 杂项和工具

- uuid - UUID 实现.
- itertools - 额外的迭代器函数和宏。
- maplit - hashmap!{} 宏， 类似于 vec![]
- cfg-if - 用来简化互斥条件编译标签(#[cfg])的宏。
- just - 更好的 make.
