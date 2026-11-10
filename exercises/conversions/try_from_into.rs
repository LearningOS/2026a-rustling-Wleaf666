// try_from_into.rs
//
// TryFrom 是一种简单且安全的类型转换，在某些情况下可能会以受控方式失败。
// 基本上它与 From 相同。主要区别在于它应该返回一个 Result 类型，而不是目标类型本身。
// 你可以在这里阅读更多内容：https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// 执行 `rustlings hint try_from_into` 或在 watch 模式下使用 `hint` 子命令以获取提示。

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// 在下面这些 TryFrom 转换中使用此错误类型。
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // 切片长度不正确。
    BadLen,
    // 整数转换失败。
    IntConversion,
}


// 你的任务是补全下面的实现，成功时返回内部值为 Color 的 Ok 结果。
// 需要分别为三种输入实现转换：包含三个整数的元组、
// 包含三个整数的数组，以及整数切片。
//
// 注意：元组和数组的元素数量可在编译时确定，
// 但切片的长度需要在运行时检查！另外，
// 有效的 RGB 颜色分量必须是 0..=255 范围内的整数。

// 元组的转换实现。
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (r,g,b)=tuple;
        if r>=0 && r<=255 &&g>=0 && g<=255 && b>=0 && b<=255{
            return Ok(Color { red: r as u8, green: g as u8, blue: b as u8 })
        }
        Err(IntoColorError::IntConversion)
    }
}

// 数组的转换实现。
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let r= arr[0];
        let g= arr[1];
        let b= arr[2];
        if r>=0 && r<=255 &&g>=0 && g<=255 && b>=0 && b<=255{
            return Ok(Color { red: r as u8, green: g as u8, blue: b as u8 })
        }
        Err(IntoColorError::IntConversion)
    }
}

// 切片的转换实现。
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        let mut it = slice.into_iter();
        match (it.next(),it.next(),it.next(),it.next()){
            (Some(&r),Some(&g),Some(&b),None) =>{
                if r>=0 && r<=255 &&g>=0 && g<=255 && b>=0 && b<=255{
                    return Ok(Color { red: r as u8, green: g as u8, blue: b as u8 })
                }
                Err(IntoColorError::IntConversion)
            }
            _ =>Err(IntoColorError::BadLen)
        }
    }
}

fn main() {
    // 使用 try_from 函数进行转换。
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // 已经为 Color 实现了 TryFrom，因此也可以使用 TryInto 进行同方向的转换。
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // 对于切片，可以使用 try_from 函数进行转换。
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // 也可以用圆括号括住切片表达式，再调用 try_into()。
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
