// clippy3.rs
// 
// 这里还有几个简单的 Clippy 修复，让你体会它的实用性。
//
// 执行 `rustlings hint clippy3` 或在 watch 模式下使用 `hint` 子命令以获取提示。


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;


    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a,& mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
