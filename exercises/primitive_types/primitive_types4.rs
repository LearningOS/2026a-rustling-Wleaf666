// primitive_types4.rs
//
// 在 ??? 处获取数组 a 的切片，使测试通过。
//
// 执行 `rustlings hint primitive_types4` 或在 watch 模式下使用 `hint` 子命令以获取提示。



#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];
    assert_eq!([2, 3, 4], nice_slice)
}
