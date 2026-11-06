// lifetimes3.rs
//
// 当结构体持有引用时，也需要指定生命周期。
//
// 执行 `rustlings hint lifetimes3` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

struct Book <'a>{
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
