// macros3.rs
//
// 在不将宏移出模块的前提下，让代码通过编译！
//
// 执行 `rustlings hint macros3` 或在 watch 模式下使用 `hint` 子命令以获取提示。

#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
