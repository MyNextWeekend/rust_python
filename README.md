# rust_python
使用pyo3为python编写包

使用rye作为管理工具
```shell
# 初始化项目
rye init --build-system maturin
# 设置python版本
rye pin 3.9.7
# 环境中添加python包
rye add requests
# 构建rust / 拉取python包
rye sync 

```