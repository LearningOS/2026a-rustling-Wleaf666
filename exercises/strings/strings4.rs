// strings4.rs
//
// 好的，这里有一组值——有些是 `String`，有些是 `&str`。你的任务是根据每个值的类型，
// 在每行括号前调用这两个函数之一（添加 `string_slice` 或 `string`）。
// 如果你的判断是正确的，代码就会编译通过！
//
// 这次没有提示！

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
