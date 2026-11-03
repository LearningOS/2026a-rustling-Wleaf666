// tests9.rs
//
// Rust 具备与 C/C++ 以及其他静态编译语言共享 FFI 接口的强大能力，甚至可以在代码内部直接链接！
// 就像下面的代码一样，这是通过 extern 块实现的。
//
// `extern` 关键字后面的短字符串指示外部导入的函数遵循哪种 ABI。在本练习中使用的是 "Rust"，
// 也有其他变体，比如标准 C ABI 的 "C"、Windows ABI 的 "stdcall"。
//
// 外部导入的函数声明在 extern 块中，用分号标记签名的结束，而不是花括号。
// 可以将一些属性应用于这些函数声明以修改链接行为，例如用 #[link_name = ".."] 修改实际符号名称。
//
// 如果你想将符号导出到链接环境，也可以在函数定义前加上带相同 ABI 字符串标注的 `extern` 关键字。
// Rust 函数的默认 ABI 本身就是 "Rust"，因此如果要链接纯 Rust 函数，可以省略整个 extern 声明。
//
// 与 C++ 类似，Rust 默认会对符号进行混淆（mangle）。为了禁用此行为并使这些函数可以通过名称定位，
// 可以应用属性 #[no_mangle]。
//
// 在本练习中，你的任务是使测试用例能够调用模块 Foo 中的 `my_demo_function`。
// `my_demo_function_alias` 是 `my_demo_function` 的别名，因此测试用例中的两行代码应该调用相同的函数。
//
// 除了添加两行属性外，你不应修改任何现有代码。

// I AM NOT DONE

extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
