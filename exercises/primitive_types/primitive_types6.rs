// primitive_types6.rs
//
// 使用元组索引访问 `numbers` 的第二个元素。你可以将访问第二个元素的表达式填在 ??? 处，使测试通过。
//
// 执行 `rustlings hint primitive_types6` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = ???;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
