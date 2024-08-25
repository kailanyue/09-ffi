
[Rye: a Hassle-Free Python Experience](https://rye.astral.sh/)

### 1 安装 Rye

```sh
# use binary file install
curl -sSf https://rye.astral.sh/get | bash

# use source code install
cargo install --git https://github.com/astral-sh/rye rye
```

### 2 创建项目添加依赖

```sh
# 创建项目
rye init python-binding --build-system maturin
rye sync
rye install maturin
rye add --dev pip
rye add --dev ipython
```

### 3 编译运行项目
```sh
maturin develop
rye run ipython
```


### 4 在python中使用

```python
In [1]: from algo import  Algo

In [2]: a = Algo("blake3")

In [3]: a.hash('hello world')
Out[3]: 'd74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24'

In [4]: a.get_name()
Out[4]: 'blake3'
```

###  如果修改项目名

要点：
- pyproject.toml 中 project 名称，module-name
- `python/python-binding` 改为 `python/algo`
- __init__.py 中 import 名称一致
- 新撰写的 function / class 记得在 lib.rs 下引入，并且在 __init__.py 中引入
- 使用 magic function 使得代码更加 python 化
