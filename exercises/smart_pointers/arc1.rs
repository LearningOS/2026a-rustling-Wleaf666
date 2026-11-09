// arc1.rs
//
// 在本练习中，给出了一个名为 "numbers" 的 u32 类型的 Vec，其值范围从 0 到 99 —— [ 0, 1, 2, ..., 98, 99 ]。
// 我们希望在 8 个不同的线程中同时使用这组数字。每个线程将计算带有偏移量的每第 8 个值的总和。
//
// 第 1 个线程（偏移量 0）将计算 0, 8, 16, ... 的和
// 第 2 个线程（偏移量 1）将计算 1, 9, 17, ... 的和
// 第 3 个线程（偏移量 2）将计算 2, 10, 18, ... 的和
// ...
// 第 8 个线程（偏移量 7）将计算 7, 15, 23, ... 的和
//
// 因为我们正在使用线程，所以我们的值必须是线程安全的。因此，我们使用 Arc。
// 我们需要在两处 TODO 中分别做出修改。
//
// 在第一个 TODO 注释处为 `shared_numbers` 赋值，在第二个 TODO 注释处为 `child_numbers` 创建初始绑定，
// 从而使代码能够编译。尽量不要创建 `numbers` Vec 的任何副本！
//
// 执行 `rustlings hint arc1` 或在 watch 模式下使用 `hint` 子命令以获取提示。


#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
