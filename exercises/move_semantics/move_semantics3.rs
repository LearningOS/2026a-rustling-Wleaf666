// move_semantics3.rs
//
// 不添加新行，仅通过修改现有行使代码编译！（不需要包含多个分号的代码行！）
//
// 执行 `rustlings hint move_semantics3` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
