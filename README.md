# rust_python 混合项目

- 使用pyo3为python编写包
- 使用rye作为管理工具


## 初始化一个新的rust-python混合项目，并指定使用maturin作为构建工具
```bash
mkdir xxx
rye init --build-system maturin
```

## 项目添加python第三方包
```bash
# 设置Python版本为3.9.7（可根据项目需求修改）
rye pin 3.9.7

# 添加Python依赖包，比如：requests库
rye add requests

# 拉取并安装所有的Python依赖包
rye sync 
```

## 通过rust代码构建python包
```bash
# 安装全局的maturin工具
# maturin用于构建Rust项目并生成可供Python使用的扩展模块
rye install maturin

# 配置环境变量，将rye工具的路径添加到系统的环境变量中
vim ～/.bash_profile 
# 在.bash_profile中增加以下行：
source "$HOME/.rye/env"

# 使修改后的环境变量立即生效
source .bash_profile 
```

```bash
# 构建Rust代码，并使Python能够使用最新构建的包
# --skip-install参数用于跳过安装环节（仅用于本地开发）
maturin develop --skip-install
```
