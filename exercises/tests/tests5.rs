// tests5.rs
//
// Rust 中的 `unsafe` 充当一种契约。
//
// 当在项声明（如函数、trait 等）上标记 `unsafe` 时，它同时声明了一份契约。然而，
// 契约的具体内容不能仅通过单个关键字来表达。因此，你有责任在项的文档注释中的 `# Safety` 章节手动说明。
//
// 当在花括号包围的代码块上标记 `unsafe` 时，它声明了对某些契约的遵守，例如某些指针参数的有效性、
// 某些内存地址的所有权等。但与上述相同，你仍需要在代码块的注释中说明契约是如何被遵守的。
//
// 注意：所有这些注释都是为了你代码的可读性和可维护性，而 Rust 编译器则将代码健全性（soundness）的信任交给了你自己！
// 如果你无法证明自己代码的内存安全性和健全性，请退一步，改用安全（safe）代码！
//
// 执行 `rustlings hint tests5` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    unsafe {
        todo!("Your code goes here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
