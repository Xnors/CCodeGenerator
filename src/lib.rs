//! 这是一个通过Rust的树形结构生成C语言代码的库。
//! 类似于LLVM的IR，可以将树形结构转换为C语言代码。

pub mod core;
pub use core::Context;
pub use core::ctypes::{ CFunction, c_type, cvartypes };
