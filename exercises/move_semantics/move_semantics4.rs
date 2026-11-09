// move_semantics4.rs
//
// 重构此代码：不再将 `vec0` 传入 `fill_vec` 函数，而是在该函数内部创建 Vector 并将其返回给 main 函数。
//
// 执行 `rustlings hint move_semantics4` 或在 watch 模式下使用 `hint` 子命令以获取提示。



fn main() {

    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
