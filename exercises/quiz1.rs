// quiz1.rs
//
// 这是针对以下章节的测验：
// - 变量 (Variables)
// - 函数 (Functions)
// - 条件语句 (If)
//
// 玛丽正在买苹果。苹果的价格计算方式如下：
// - 每个苹果售价 2 rustbucks。
// - 如果玛丽购买超过 40 个苹果，每个苹果的价格立减为 1 rustbuck！
// 请编写一个函数，根据购买的苹果数量计算订单的总价。
//
// 这次没有提示哦 ;)

// I AM NOT DONE

// Put your function here!
// fn calculate_price_of_apples {

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
