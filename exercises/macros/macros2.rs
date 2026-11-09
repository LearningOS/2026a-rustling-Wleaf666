// macros2.rs
//
// 执行 `rustlings hint macros2` 或在 watch 模式下使用 `hint` 子命令以获取提示。

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}


fn main() {
    my_macro!();
}

