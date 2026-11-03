// functions2.rs
//
// 执行 `rustlings hint functions2` 或在 watch 模式下使用 `hint` 子命令以获取提示。

fn main() {
    call_me(3);
}

fn call_me(num:i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
