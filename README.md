---
# rust_python 混合项目
-------------

> 关于我  
博客：[无](http://) 

仓库主要用于学习，随便记录一些东西
- 使用pyo3为python编写包
- 使用uv作为管理工具


## 使用maturin初始化一个新的rust-python混合项目
```bash
mkdir xxx
# 初始化的时候选择pyo3
uvx maturin init
```

## 项目添加python第三方包
```bash
# 设置Python版本为3.11.10（可根据项目需求修改）
uv python pin 3.11.10

# 添加Python依赖包，比如：requests库
uv add requests

# 拉取并安装所有的Python依赖包
uv sync 
```

### uv安装全局的maturin工具
```bash
# 安装全局的maturin工具
# maturin用于构建Rust项目并生成可供Python使用的扩展模块
uv tool install maturin
```

## 通过maturin构建rust并为当前python环境安装
```bash
# 构建Rust代码，并使Python能够使用最新构建的包
uvx maturin develop -r

# 构建你的项目并生成 .whl 文件 .pyi 文件将自动放在 target/wheels/ 目录下 通常会与生成的 .whl 文件一起生成
uvx maturin build --release
```
