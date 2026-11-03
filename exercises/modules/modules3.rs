// modules3.rs
//
// 你可以使用 'use' 关键字将任何模块中的路径引入作用域，尤其是 Rust 标准库中的模块。
// 从 std::time 模块引入 SystemTime 和 UNIX_EPOCH。如果你能在一行内完成，可获得额外加分！
//
// 执行 `rustlings hint modules3` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

// TODO: Complete this use statement
use std::time::{SystemTime,UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
