// tests3.rs
//
// 这个测试没有真正在测试我们的函数——以使测试通过的方式修改它。
// 然后编写第二个测试，测试当我们调用 `is_even(5)` 时是否获得了预期的结果。
//
// 执行 `rustlings hint tests3` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!();
    }

    #[test]
    fn is_false_when_odd() {
        assert!();
    }
}
