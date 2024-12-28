//! 本文件提供了 CFunction 结构体的定义，用于描述 C 语言函数的相关信息。

use crate::c_type;

/// CFunction 结构体用于描述 C 语言函数的相关信息。
pub struct CFunction {
    name: String,

    return_type: c_type,
    parameters: Vec<c_type>,

    generated_function_body_ccode: String,
}

impl CFunction {
    /// 创建一个新的 CFunction 实例。
    ///
    /// # 参数
    ///
    /// * `name` - 函数名。
    /// * `return_type` - 函数返回值类型。
    /// * `parameters` - 函数参数列表。
    ///
    /// # 示例
    ///
    /// ```
    /// use ccgenor::CFunction;
    /// use ccgenor::cvartypes;
    ///
    /// let func = CFunction::new(
    ///     "add".to_string(),
    ///     cvartypes::C_INT,
    ///     vec!["int a", "int b"]
    /// );
    /// ```
    pub fn new(name: String, return_type: c_type, parameters: Vec<c_type>) -> Self {
        Self {
            name,
            return_type,
            parameters,
            generated_function_body_ccode: String::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_return_type(&self) -> &c_type {
        &self.return_type
    }

    pub fn get_parameters(&self) -> &[c_type] {
        &self.parameters
    }

    pub fn get_generated_function_body_ccode(&self) -> &str {
        &self.generated_function_body_ccode
    }

    pub fn set_generated_function_body_ccode(&mut self, ccode: String) {
        self.generated_function_body_ccode = ccode;
    }

    pub fn append_generated_function_body_ccode(&mut self, ccode: String) {
        self.generated_function_body_ccode += &ccode;
    }

    /// 创建函数声明
    /// # 示例
    /// ```
    /// use ccgenor::CFunction;
    /// use ccgenor::cvartypes;
    ///
    /// let mut func = CFunction::new(
    ///     "add".to_string(),
    ///     cvartypes::C_INT,
    ///     vec!["int a", "int b"]
    /// );
    ///
    /// func.create_function_declarations();
    ///
    /// assert_eq!("int add(int a, int b);", func.get_generated_function_body_ccode());
    /// ```
    pub fn create_function_declarations(&mut self) {
        let mut declarations = String::new();
        declarations.push_str(self.return_type);
        declarations.push_str(" ");
        declarations.push_str(&self.name);
        declarations.push_str("(");
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                declarations.push_str(", ");
            }
            declarations.push_str(param);
        }
        declarations.push_str(");");

        self.set_generated_function_body_ccode(declarations);
    }

    /// 创建函数体开头
    ///
    /// # 示例
    /// ```
    /// use ccgenor::CFunction;
    /// use ccgenor::cvartypes;
    ///
    /// let mut func = CFunction::new(
    ///     "add".to_string(),
    ///     cvartypes::C_INT,
    ///     vec!["int a", "int b"]
    /// );
    ///
    /// func.create_function_start();
    ///
    /// assert_eq!("int add(int a, int b) {\n", func.get_generated_function_body_ccode());
    /// ```
    pub fn create_function_start(&mut self) {
        let mut declarations = String::new();
        declarations.push_str(self.return_type);
        declarations.push_str(" ");
        declarations.push_str(&self.name);
        declarations.push_str("(");
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                declarations.push_str(", ");
            }
            declarations.push_str(param);
        }
        declarations.push_str(") {\n");

        self.set_generated_function_body_ccode(declarations);
    }

    /// 用于在函数体中调用另一个函数。
    ///
    /// # 示例
    /// 1. 调用一个普通函数
    /// ```
    /// use ccgenor::CFunction;
    /// use ccgenor::cvartypes;
    ///
    /// let mut func = CFunction::new(
    ///     "add".to_string(),
    ///     cvartypes::C_INT,
    ///     vec!["int a", "int b"]
    /// );
    ///
    /// func.create_function_call("add_long_long_ints", &["1000000000000", "2000000000000"]);
    ///
    /// assert_eq!("add_long_long_ints(1000000000000, 2000000000000);\n", func.get_generated_function_body_ccode());
    /// ```
    ///
    /// 2. 调用一个内置函数
    /// ```
    /// use ccgenor::CFunction;
    /// use ccgenor::cvartypes;
    ///
    /// let mut func = CFunction::new(
    ///     "add".to_string(),
    ///     cvartypes::C_INT,
    ///     vec!["int a", "int b"]
    /// );
    ///
    /// func.create_function_call("printf", &["\"%d + %d = %d\\n\"", "a", "b"]);
    ///
    /// assert_eq!("printf(\"%d + %d = %d\\n\", a, b);\n", func.get_generated_function_body_ccode());
    /// ```
    pub fn create_function_call(&mut self, function_name: &str, args: &[&str]) {
        let mut call = String::new();
        call.push_str(function_name);
        call.push_str("(");
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                call.push_str(", ");
            }
            call.push_str(arg);
        }
        call.push_str(");\n");

        self.append_generated_function_body_ccode(call);
    }

    /// 创建函数体结尾
    ///
    /// # 示例
    /// ```
    /// use ccgenor::CFunction;
    /// use ccgenor::cvartypes;
    ///
    /// let mut func = CFunction::new(
    ///     "add".to_string(),
    ///     cvartypes::C_INT,
    ///     vec!["int a", "int b"]
    /// );
    ///
    /// func.create_function_end();
    ///
    /// assert_eq!("}\n", func.get_generated_function_body_ccode());
    /// ```
    pub fn create_function_end(&mut self) {
        self.append_generated_function_body_ccode("}\n".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cvartypes;

    #[test]
    fn test_new() {
        // 测试创建函数实例
        let func = CFunction::new(
            "add".to_string(),
            cvartypes::C_INT,
            vec!["int a", "int b"]
        );
        assert_eq!("add", func.get_name());
        assert_eq!(cvartypes::C_INT, *func.get_return_type());
        assert_eq!(2, func.get_parameters().len());
        assert_eq!("int a", func.get_parameters()[0]);
        assert_eq!("int b", func.get_parameters()[1]);
        assert_eq!("", func.get_generated_function_body_ccode());
    }

    #[test]
    fn test_set_generated_function_body_ccode() {
        let mut func = CFunction::new(
            "add".to_string(),
            cvartypes::C_INT,
            vec!["int a", "int b"]
        );
        func.set_generated_function_body_ccode("return a + b;".to_string());
        assert_eq!("return a + b;", func.get_generated_function_body_ccode());
    }

    #[test]
    fn test_create_function_declarations1() {
        let mut func = CFunction::new(
            "add".to_string(),
            cvartypes::C_INT,
            vec!["int a", "int b"]
        );
        func.create_function_declarations();
        assert_eq!("int add(int a, int b);", func.get_generated_function_body_ccode());
    }

    #[test]
    fn test_create_function_declarations2() {
        let mut func = CFunction::new(
            "add_long_long_ints".to_string(),
            "long long int",
            vec!["long long int", "long long int"]
        );
        func.create_function_declarations();
        assert_eq!(
            "long long int add_long_long_ints(long long int, long long int);",
            func.get_generated_function_body_ccode()
        );
    }
}
