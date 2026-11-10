// tests8.rs
//
// 本练习与上一个练习共享 `build.rs`。
// 你需要在 `build.rs` 中添加一些代码，使本练习和上一个练习都能正常工作。
//
// 执行 `rustlings hint tests8` 或在 watch 模式下使用 `hint` 子命令以获取提示。


fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
