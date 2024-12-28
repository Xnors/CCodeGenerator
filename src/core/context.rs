//! 本文件提供了一个Context结构，用于存储生成的C语言代码。
//! 是树的根节点

use crate::CFunction;

/// Context 上下文
/// 用于存储生成的C语言代码
///
/// # Examples
///
/// ```
/// use ccgenor::Context;
///
/// let mut ctx = Context::new();
///
/// // 一些生成代码的操作
///
/// let ccode = ctx.get_ccode();
///
/// println!("{}", ccode);
/// ```
pub struct Context {
    ccode: String,
}
impl Context {
    pub fn new() -> Self {
        Self {
            ccode: String::from(""),
        }
    }
    pub fn get_ccode(&self) -> &str {
        &self.ccode
    }
    pub fn add_ccode(&mut self, ccode: &str) {
        self.ccode.push_str(ccode);
    }

    /// 添加头文件
    /// # Examples
    /// ```
    /// use ccgenor::Context;
    ///
    /// let mut ctx = Context::new();
    /// ctx.add_include("stdio.h");
    /// assert_eq!(ctx.get_ccode(), "#include \"stdio.h\"\n");
    /// ```
    pub fn add_include(&mut self, head_file_to_include: &str) {
        self.add_ccode(&format!("#include \"{}\"\n", head_file_to_include));
    }

    /// 添加函数
    /// # Examples
    /// ```
    /// use ccgenor::Context;
    /// use ccgenor::CFunction;
    ///
    /// let mut ctx = Context::new();
    /// let mut func = CFunction::new("test_func".to_string(), "int", vec!["int a", "int b"]);
    /// func.create_function_start();
    /// func.create_function_end();
    /// ctx.add_function(&func);
    /// assert_eq!(ctx.get_ccode(), "int test_func(int a, int b) {\n}\n");
    /// ```
    pub fn add_function(&mut self, func: &CFunction) {
        self.add_ccode(&func.get_generated_function_body_ccode());
    }

    /// 打印生成的C语言代码
    pub fn print_ccode(&self) {
        println!("{}", self.ccode);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_context() {
        let mut ctx = Context::new();
        ctx.add_ccode("int a = 10;\n");
        ctx.add_ccode("int b = 20;\n");
        assert_eq!(ctx.get_ccode(), "int a = 10;\nint b = 20;\n");
    }

    #[test]
    fn test_include() {
        let mut ctx = Context::new();
        ctx.add_include("stdio.h");
        assert_eq!(ctx.get_ccode(), "#include \"stdio.h\"\n");
    }

    #[test]
    fn test_function() {
        let mut ctx = Context::new();
        let mut func = CFunction::new("test_func".to_string(), "int", vec!["int a", "int b"]);
        func.create_function_start();
        func.create_function_end();
        ctx.add_function(&func);
        assert_eq!(ctx.get_ccode(), "int test_func(int a, int b) {\n}\n");
    }
}
