## roadmap

 - ✅ 已实现
 - 🚧 实现中
 - ❌ 未实现
 - 🛠️ 需重构/修复

| 功能                 | 模块路径                              | 文件路径               | 特性          | 状态 |
| -------------------- | ------------------------------------- | ---------------------- | ------------- | :--: |
| 获取窗口信息         | `window`<br />`crate::window::info`   | *src/window.rs*        | *prelude::\** |  ✅   |
| 获取窗口样式         | `window`<br />`crate::window::style`  | *src/window/style.rs*  | *prelude::\** |  ✅   |
| 发送窗口消息         | `window`<br />`crate::window::msg`    | *src/window/msg.rs*    | *prelude::\** |  🚧   |
| 获取窗口消息         | `window`<br />`crate::window::msg`    | *src/window/msg.rs*    | *prelude::\** |  ❌   |
| 激活窗口             | `window`<br />`crate::window::active` | *src/window/active.rs* | *prelude::\** |  🚧   |
| 获取屏幕信息         | `monitor`<br />`crate::monitor::info` | *src/monitor/info.rs*  |               |  🚧   |
| 启动进程并获取主窗口 | `window`<br />`crate::window::active` | *src/window/active.rs* | *prelude::\** |  ❌   |
| 解析命令             | `window::parser`                      | *src/parser.rs*        | *parser*      |  🚧   |
| 解析并执行命令       | `window::evaluate`                    | *src/evaluate.rs*      | *eval*        |  🚧   |



## 依赖协议声明

本项目依赖了一些第三方 Rust crates，具体依赖及其协议请见 *Cargo.toml* 和各 crate 的 *LICENSE* 文件。项目主要依赖均使用 MIT 或 Apache-2.0 等宽松协议。

可以使用 [`cargo-license`](https://github.com/onur/cargo-license) 工具，快速查看本项目所有依赖的许可证信息：

```sh
cargo install cargo-license
cargo license --json
```
