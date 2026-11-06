// errors5.rs
//
// 本程序使用了 errors4 中代码的修改版本。
//
// 本练习使用了一些我们在课程后半部分才会涉及的概念，例如 `Box` 和 `From` trait。
// 现在不需要详细理解它们，如果你愿意，可以提前阅读相关章节。目前，你可以将
// `Box<dyn ???>` 类型理解为“只要实现了 ??? trait 的任何类型我都接受”，鉴于 Rust
// 平时在运行时安全方面的严格标准，这听起来应该会显得有些宽松！
//
// 简而言之，Box 的这种特定用法适用于你想要拥有一个值的所有权，但只关心它实现了某个特定 trait 的情况。
// 为此，Box 被声明为 Box<dyn Trait> 类型，其中 Trait 是编译器在该上下文中查找的 trait。
// 在本练习中，该上下文是 Result 中可能返回的潜在错误。
//
// 我们可以用什么来同时描述这两种错误？换句话说，是否存在这两种错误都实现了的 trait？
//
// 执行 `rustlings hint errors5` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn  std::error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
