# 安装 Rust

> 📖 对应书中章节：Chapter 1.1 - Installation

本章将介绍如何在 Linux、macOS 和 Windows 上安装 Rust。

---

## 📦 什么是 rustup？

`rustup` 是 Rust 官方推荐的**版本管理工具**，它可以：
- 安装和管理多个 Rust 版本
- 在稳定版(stable)、测试版(beta)、 nightly版之间切换
- 更新 Rust 到最新版本
- 管理额外的组件（如 rustfmt、clippy 等）

> 💡 类比理解：`rustup` 就像是 Node.js 的 `nvm` 或 Python 的 `pyenv`

---

## 🖥️ 在 Linux/macOS 上安装

打开终端，运行以下命令：

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

安装成功后，你会看到：

```
Rust is installed now. Great!
```

### 🔗 安装链接器

Rust 编译器需要一个**链接器(linker)**来将编译后的代码连接成可执行文件。

- **macOS**：安装 Xcode 命令行工具
  ```bash
  xcode-select --install
  ```

- **Linux (Ubuntu/Debian)**：安装 build-essential
  ```bash
  sudo apt install build-essential
  ```

- **Linux (Fedora/CentOS)**：安装开发工具组
  ```bash
  sudo dnf install gcc
  # 或
  sudo yum groupinstall "Development Tools"
  ```

---

## 🪟 在 Windows 上安装

1. 访问 [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. 下载并运行 `rustup-init.exe`
3. 安装过程中会提示安装 **Visual Studio Build Tools**（提供链接器和 Windows SDK）

> ⚠️ Windows 用户需要安装 Visual Studio 的 C++ 开发工具，否则编译会失败

---

## ✅ 验证安装

安装完成后，打开**新的终端窗口**，运行：

```bash
rustc --version
```

你应该看到类似输出：

```
rustc 1.90.0 (xxxxxxx 2026-01-15)
```

如果看到版本号，恭喜你，Rust 安装成功！

---

## 🔄 常用命令速查

| 命令 | 说明 |
|------|------|
| `rustc --version` | 查看 Rust 编译器版本 |
| `cargo --version` | 查看 Cargo 版本 |
| `rustup update` | 更新 Rust 到最新版本 |
| `rustup self uninstall` | 卸载 Rust 和 rustup |
| `rustup doc` | 在浏览器中打开本地文档 |
| `rustup doc --book` | 打开本地版《The Rust Programming Language》 |

---

## 📚 本地文档

Rust 安装时会同时安装完整的离线文档：

```bash
# 打开 Rust 标准库文档
rustup doc

# 打开官方书籍（推荐！）
rustup doc --book
```

> 💡 不确定某个标准库函数怎么用？用 `rustup doc` 查离线文档，无需联网！

---

## 🛠️ 推荐的编辑器/IDE

虽然任何文本编辑器都能写 Rust，但以下工具对 Rust 支持最好：

| 编辑器/IDE | Rust 支持 |
|------------|-----------|
| VS Code | 安装 `rust-analyzer` 插件（官方推荐） |
| RustRover | JetBrains 专为 Rust 开发的 IDE |
| CLion | JetBrains 插件支持 |
| Neovim/Vim | 配合 `rust-analyzer` LSP |

> 💡 `rust-analyzer` 是官方主导开发的 Rust 语言服务器，提供代码补全、跳转定义、错误提示等功能

---

## ❓ 常见问题

### Q: 提示 `command not found: rustc`

**原因**：Rust 的路径没有被添加到 PATH 环境变量

**解决**：
- Linux/macOS：重启终端，或手动执行 `source $HOME/.cargo/env`
- Windows：重新打开终端，或检查 `%USERPROFILE%\.cargo\bin` 是否在 PATH 中

### Q: 编译时报链接器错误

**原因**：系统缺少 C 编译器/链接器

**解决**：
- macOS：运行 `xcode-select --install`
- Linux：安装 gcc 或 clang
- Windows：安装 Visual Studio Build Tools

---

## 🎯 下一步

安装完成后，让我们来编写第一个 Rust 程序——经典的 "Hello, World!" → [02_hello_world.md](./02_hello_world.md)
