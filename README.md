# rust_python 混合项目

---

> 关于我
> 博客：[无](http://)

本仓库主要用于学习，记录 Rust 和 Python 混合开发的相关内容。

## 项目特点

- 使用 **pyo3** 为 Python 编写扩展包
- 使用 **uv** 作为项目管理工具，**maturin** 作为构建工具
- 集成 GitHub 自动构建流水线

---

## 初始化项目

```bash
mkdir 项目名 && cd 项目名
# 初始化时选择 maturin 作为构建后端
uv init --build-backend maturin
```

## 添加 Python 扩展包

```bash
# 设置 Python 版本为 3.11.10（可根据项目需求修改）
uv python pin 3.11.10

# 添加 Python 依赖包，例如 requests 库
uv add requests

# 拉取并安装所有的 Python 依赖包
uv sync
```

## 安装全局 maturin 工具

```bash
# 安装全局 maturin 工具
# maturin 用于构建 Rust 项目并生成可供 Python 使用的扩展模块
uv tool install maturin

# 删除全局 maturin 工具
uv tool uninstall maturin
```

## 构建 wheel 并安装到当前环境中

```bash
# 构建 Rust 并安装到当前 Python 环境中
maturin dev --uv

# 构建项目并生成生产环境的 .whl 文件
maturin build --release --strip
```

## 使用 GitHub Actions 构建跨平台 wheel 包

```bash
# 示例见文件夹 .github/workflows
```
