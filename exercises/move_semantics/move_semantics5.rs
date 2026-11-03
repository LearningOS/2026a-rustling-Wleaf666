// move_semantics5.rs
//
// 仅通过重新排列 `main()` 中的代码行使其编译，不要添加、修改或删除任何行。
//
// 执行 `rustlings hint move_semantics5` 或在 watch 模式下使用 `hint` 子命令以获取提示。

fn main() {
    let mut x = 100;
    {
        let y = &mut x;
        *y += 100;
    }
    
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
