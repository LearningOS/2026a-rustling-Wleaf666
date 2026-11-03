// functions4.rs
//
// 这家商店正在打折：如果原价是偶数，立减 10 Rustbucks；
// 如果是奇数，立减 3 Rustbucks。（先不用担心函数体本身，
// 我们目前只关注函数签名。无论如何，这是提前预览后续练习的好方法！）
//
// 执行 `rustlings hint functions4` 或在 watch 模式下使用 `hint` 子命令以获取提示。

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32{
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
