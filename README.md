# rust-exercise

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
