// iterators1.rs
//
// 对集合内的元素执行操作时，迭代器至关重要。
// 本模块帮助你熟悉使用迭代器的结构以及如何遍历可迭代集合内的元素。
//
// 填补 `???` 处的代码使程序编译通过！
//
// 执行 `rustlings hint iterators1` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: Step 4
}
