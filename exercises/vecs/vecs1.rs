// vecs1.rs
//
// 你的任务是创建一个 `Vec`，使其包含与数组 `a` 完全相同的元素。
//
// 让代码通过编译并使测试通过！
//
// 执行 `rustlings hint vecs1` 或在 watch 模式下使用 `hint` 子命令以获取提示。



fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = vec![10,20,30,40];// TODO: declare your vector here with the macro for vectors

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
