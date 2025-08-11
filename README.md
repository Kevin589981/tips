# Tips - 快捷记笔记工具

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个简洁的命令行笔记工具，支持快速创建临时笔记文件，关闭编辑器后自动清理文件。

## 功能特点

- 🚀 快速启动编辑器创建临时笔记
- 🧹 编辑完成后自动清理临时文件
- ⚙️ 可配置文件类型和编辑器
- 📝 使用时间戳命名文件避免冲突
- 🏠 配置文件存储在用户目录

## 安装

```bash
cargo build --release
```

## 使用方法
- 在Windows机器下使用不同编辑器可能需要在源代码删除--waiting参数
1. 首次运行时会自动创建配置文件 `~/.tips.toml`
2. 配置文件内容：
   ```toml
   file_type = "txt"  # 文件扩展名
   editor = "code.cmd"    # 编辑器命令
   ```
3. 运行 `tips` 启动编辑器创建临时笔记
4. 保存并关闭编辑器后，临时文件会自动删除


## 工作原理

1. 在用户目录下创建 `tips_temp` 文件夹
2. 生成带时间戳的临时文件名
3. 启动配置的编辑器并等待编辑完成
4. 编辑器退出后自动删除临时文件

## 依赖

- Rust 2024 Edition
- tokio - 异步运行时
- serde/toml - 配置文件处理
- chrono - 时间戳生成
- directories - 用户目录获取

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件