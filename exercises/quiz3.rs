// quiz3.rs
//
// 本测验测试以下内容：
// - 泛型 (Generics)
// - 特征 (Traits)
//
// 一所虚拟的魔法学校拥有一个用 Rust 编写的新成绩单生成系统！
// 目前该系统仅支持生成以数字表示学生成绩的成绩单（例如 1.0 -> 5.5）。
// 然而，学校还发放字母等级（A+ -> F-），并且需要能够打印这两种类型的成绩单！
//
// 在 ReportCard 结构体和 impl 块中进行必要的代码修改以支持字母成绩单。
// 将第二个测试中的 Grade 改为 "A+"，以表明你的修改支持字母等级。
//
// 执行 `rustlings hint quiz3` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
