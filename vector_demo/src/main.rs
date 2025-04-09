use Vec;
// 单元格枚举
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //声明一个集合
    let mut vec2 = Vec::new();
    // 给集合赋值
    vec2.push(1);
    vec2.push(2);
    vec2.push(3);
    vec2.push(4);
    // 获取集合中的数据
    println!("通过下标获取第三个值：{}", vec2[2]);
    //通过get获取值

    let index = 20;
    match vec2.get(index) {
        Some(third) => println!("下标为{}个值是{}", index, third),
        None => println!("没有找到下标为{}的值", index),
    }

    //循环

    for i in &vec2 {
        println!("{}", i);
    }

    let s1 = String::from("hello");
    let s2 = String::from("world");

    let s3 = s1 + &s2;
    println!("{}", s3);
}
