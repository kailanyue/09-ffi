### 1 Matrix
Matrix 代码的 Python 封装实现

#### 1.1 在 algo 中添加 Matrix 的代码
将已有的 [matrix.rs](../algo/src/matrix.rs) 代码添加到 `algo` module

#### 1.2 引入到 python-binding 中

##### 1.2.1 创建 PyMatrix 和 对应方法
在 python-binding 的 src 目录下创建 [matrix.rs](src/matrix.rs) 分别定义 `class` 和 `methods` 同时为 `PyMatrix` 实现 `Display`

```rust

#[pyclass(name = "Matrix")]
pub struct PyMatrix {
    inner: Matrix<f64>,
}

#[pymethods]
impl PyMatrix {
    #[new]
    pub fn try_new(data: Vec<Vec<f64>>) -> PyResult<Self> {}
    ... ...

    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }
}

impl fmt::Display for PyMatrix {}
```

##### 1.2.2 在 lib 中 add_class
在 [lib.rs](src/lib.rs) 中引入 `PyMatrix`

```rust
#[pymodule]
fn _lowlevel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_class::<PyAlgo>()?;
    m.add_class::<PyMatrix>()?;
    Ok(())
}
```

##### 1.2.3 `__init__.py` 中引入
在 [`__init__.py`](python\algo\/__init__.py) 中引入
```py
from algo._lowlevel import Algo, Matrix, hello

__all__ = [
    "Algo",
    "Matrix",
    "hello",
]

```


#### 1.3 编译运行测试
```sh
maturin develop
rye run ipython
```

运行测试代码
```py
In [1]: from algo import Matrix

In [2]: m1 = Matrix([[1, 2], [3, 4]])

In [3]: m2 = Matrix([[5, 6], [7, 8]])

In [4]: m3 = m1.mul(m2)

In [5]: m3
Out[5]: {19 22, 43 50}
```

### 2 roaring
学习使用 roaring bitmap:  https://docs.rs/roaring/latest/roaring/
使用 pyo3/napi 为其提供接口（可以任选其一），供 python / nodejs 使用

#### 2.1 将 roaring 引入到 python-binding 中
> 由于 roaring 已经提供了常用的方法，因此在本方案中，不再需要在 `algo` 为其创建更多的方法，只需要引入即可

##### 2.1.1 在 `python-binding` 添加依赖
向 [Cargo.toml](Cargo.toml) 中添加 `roaring` 依赖
```toml
[dependencies]
roaring = { version = "0.10", features = ["serde"] }
```

##### 2.1.2 创建 PyMatrix 和 对应方法
由于在 roaring 中有 RoaringBitmap 和 RoaringTreemap，因此将两个都引入，由于他们常用的方法的签名很相似，因此为了减少代码的冗余，使用宏来实现。

在 python-binding 的 src 目录下创建 [roaring.rs](src/roaring.rs)  创建宏 implement_roaring_set 来实现常用的方法

为了实现**字符串处理：在编译时处理字符串，将其转化为标识符或其他代码片段。** 引入了 `paste` 包

```rust
macro_rules! implement_roaring_set {
    ($py_name:ident, $new_name:expr, $inner_type:ty, $item_type:ty) => {
        paste::item! {
            #[pyclass(name = $new_name)]
            pub struct $py_name {
                inner: $inner_type,
            }

            #[pymethods]
            impl $py_name {
                #[new]
                fn new() -> Self {
                    Self {
                        inner: <$inner_type>::new(),
                    }
                }

                #[staticmethod]
                fn full() -> Self {}
                ... ...
                pub fn __repr__(&self) -> PyResult<String> {}

                pub fn __str__(&self) -> PyResult<String> {}
            }

            impl fmt::Display for $py_name {}
        }
    };
}

// Assuming RoaringBitmap and RoaringTreemap are already defined somewhere
implement_roaring_set!(PyBitmap, "Bitmap", RoaringBitmap, u32);
implement_roaring_set!(PyTreemap, "Treemap", RoaringTreemap, u64);

```

##### 2.1.3 在 lib 中 add_class
在 [lib.rs](src/lib.rs) 中引入 `PyMatrix`

```rust
#[pymodule]
fn _lowlevel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_class::<PyAlgo>()?;
    m.add_class::<PyMatrix>()?;
    m.add_class::<PyBitmap>()?;
    m.add_class::<PyTreemap>()?;
    Ok(())
}
```

##### 2.1.4 `__init__.py` 中引入
在 [`__init__.py`](python\algo\/__init__.py) 中引入
```py
from algo._lowlevel import Algo, Bitmap, Matrix, Treemap, hello

__all__ = [
    "Algo",
    "Matrix",
    "hello",
    "Bitmap",
    "Treemap",
]

```

#### 2.2 编译运行


```sh
maturin develop
rye run ipython
```

运行常见的测试案例
```py
In [1]: from algo import Bitmap

In [2]: from algo import Treemap

In [3]: b1 = Bitmap()

In [4]: b2 = Bitmap()

In [5]: b1.insert(1)
Out[5]: True

In [6]: b2.insert(1)
Out[6]: True

In [7]: b2.insert(1)
Out[7]: False

In [8]: b2.insert(2)
Out[8]: True

In [9]: b1.is_subset(b2)
Out[9]: True

In [10]: b2.is_superset(b1)
Out[10]: True

In [11]: b2.max()
Out[11]: 2

In [12]: b2.len()
Out[12]: 2

In [13]: t1 = Treemap()

In [14]: t1.insert(9)
Out[14]: True

In [15]: t1.max()
Out[15]: 9
```
