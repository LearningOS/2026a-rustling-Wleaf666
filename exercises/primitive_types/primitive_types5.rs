// primitive_types5.rs
//
// 解构 `cat` 元组，使 println 能够正常输出。
//
// 执行 `rustlings hint primitive_types5` 或在 watch 模式下使用 `hint` 子命令以获取提示。

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name,age) = cat;

    println!("{} is {} years old.", name, age);
}
