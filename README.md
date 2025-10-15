# RipTree（RT）

> 原版 `tree` 项目地址：[https://github.com/Old-Man-Programmer/tree](https://github.com/Old-Man-Programmer/tree)

本项目是对经典类 UNIX 命令 `tree` 的 Rust 语言重写版本。`tree` 是一个实用的小工具，可以在命令行下以树状结构显示目录内容，帮助用户快速了解文件夹的层次结构，尤其在排查隐藏文件或复杂目录时非常方便。

## 项目背景

原版 `tree` 命令由 Steve Baker 等众多贡献者开发和维护，支持多平台，功能丰富，包括彩色显示、HTML/JSON/XML 输出、符号链接处理、模式过滤等。Rust 版本旨在继承这些优秀特性，同时利用 Rust 的安全性和现代工具链优势，提升性能与可维护性。

## 功能特性

- 支持树状结构显示文件和子目录
- 兼容原版 `tree` 的大部分命令行选项
- 更高性能，内存安全
- 跨平台支持（Linux/macOS/Windows）
<!-- 彩色输出（支持`CLICOLOR`、`NO_COLOR`等环境变量）
- 递归符号链接跟踪
- 文件/目录过滤与排序
- 支持 JSON/HTML 等格式输出（计划/进行中）-->

## 安装方法

<!-- 请参考 [INSTALL.md](./INSTALL.md) 文件获取详细安装说明。使用 Rust 工具链，您可以通过 `cargo install rust-tree` 命令快速安装（待 crates.io 发布后）。-->
拉取项目代码：

```bash
git clone https://github.com/tc999/riptree.git
cd riptree
```

### 编译：

```bash
cargo build --release
```

### 安装：
- Windows: 

  把 `target/release/rt.exe` 和 `target/release/locales` 目录复制到系统 PATH 中包含的相同目录下。

- Linux:

```bash
sudo mkdir -p /opt/rt
sudo cp target/release/rt /opt/rt/rt
sudo cp -r target/release/locales /opt/rt/locales
sudo ln -s /opt/rt/rt /usr/local/bin/rt
```

## 使用方法

命令格式：
```bash
rt [选项] [目录路径（可选）]
```
### 国际化

- 支持多语言输出，默认根据系统环境变量 `LANG` 自动检测语言。
- Windows 如何设置默认显示语言：新建一个系统环境变量 `LANG`，值为 `zh-CN.UTF-8`。

常用参数举例：

<!-- - `-L <层数>` 限制显示层级深度 -->
- `-a` 显示所有文件（包括隐藏文件）
<!-- - `-C` 强制彩色显示
- `-P <模式>` 只显示匹配模式的文件 -->
- `-d` 仅显示目录
- `--LANG=<语言>` 设置输出语言
- 更多参数请使用 `rt --help` 查看

## 致谢

本项目参考并致敬原版 [`tree`](https://github.com/Old-Man-Programmer/tree) 的设计与众多贡献者，感谢 Steve Baker 及所有为 `tree` 命令做出贡献的人。Rust 版本将持续改进，欢迎大家提交 issue 和 PR，提出建议与反馈。

## 许可协议

本项目使用 GPL-3.0 协议开源，您可以在遵守协议条款的前提下自由使用、修改和分发本项目的代码。

