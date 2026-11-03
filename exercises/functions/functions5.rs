// functions5.rs
//
// 执行 `rustlings hint functions5` 或在 watch 模式下使用 `hint` 子命令以获取提示。

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
