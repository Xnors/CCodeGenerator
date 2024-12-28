pub mod cfunction;

pub use cfunction::CFunction;
pub use cvartypes::*;

/// 这个 mod 主要是为了提供一些常用的 C 类型定义，方便使用。
/// 对于一些复杂的类型，比如指针、数组、结构体等，这里暂时不提供,
/// 请使用字符切片来表示这些类型。
pub mod cvartypes {
    pub type c_type = &'static str;


    pub const C_INT: c_type = "int";
    pub const C_FLOAT: c_type = "float";
    pub const C_DOUBLE: c_type = "double";
    pub const C_CHAR: c_type = "char";
    pub const C_VOID: c_type = "void";
    pub const C_BOOL: c_type = "bool";
    pub const C_LONG: c_type = "long";
    pub const C_SHORT: c_type = "short";
    pub const C_UNSIGNED: c_type = "unsigned";
    pub const C_SIGHNED: c_type = "signed";
}
