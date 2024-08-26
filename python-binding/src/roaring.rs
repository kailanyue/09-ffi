use core::fmt;
use pyo3::prelude::*;
use roaring::{RoaringBitmap, RoaringTreemap};

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
                fn full() -> Self {
                    Self {
                        inner: <$inner_type>::full(),
                    }
                }

                fn insert(&mut self, i: $item_type) -> bool {
                    self.inner.insert(i)
                }

                pub fn push(&mut self, value: $item_type) -> bool {
                    self.inner.push(value)
                }

                pub fn remove(&mut self, value: $item_type) -> bool {
                    self.inner.remove(value)
                }

                pub fn contains(&self, value: $item_type) -> bool {
                    self.inner.contains(value)
                }

                pub fn clear(&mut self) {
                    self.inner.clear();
                }

                pub fn is_empty(&self) -> bool {
                    self.inner.is_empty()
                }

                pub fn len(&self) -> u64 {
                    self.inner.len()
                }

                pub fn min(&self) -> Option<$item_type> {
                    self.inner.min()
                }

                pub fn max(&self) -> Option<$item_type> {
                    self.inner.max()
                }

                pub fn rank(&self, value: $item_type) -> u64 {
                    self.inner.rank(value)
                }

                pub fn select(&self, n: $item_type) -> Option<$item_type> {
                    self.inner.select(n)
                }

                pub fn is_subset(&self, other: &Self) -> bool {
                    self.inner.is_subset(&other.inner)
                }

                pub fn is_superset(&self, other: &Self) -> bool {
                    self.inner.is_superset(&other.inner)
                }

                pub fn __repr__(&self) -> PyResult<String> {
                    Ok(format!("{}", self))
                }

                pub fn __str__(&self) -> PyResult<String> {
                    Ok(format!("{}", self))
                }
            }

            impl fmt::Display for $py_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}{{", stringify!($py_name))?;
                    let mut iter = self.inner.iter();
                    if let Some(first) = iter.next() {
                        write!(f, "{}", first)?;
                        for value in iter {
                            write!(f, ", {}", value)?;
                        }
                    }
                    write!(f, "}}")
                }
            }
        }
    };
}

// Assuming RoaringBitmap and RoaringTreemap are already defined somewhere
implement_roaring_set!(PyBitmap, "Bitmap", RoaringBitmap, u32);
implement_roaring_set!(PyTreemap, "Treemap", RoaringTreemap, u64);
