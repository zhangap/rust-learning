fn main() {
    // let mut s = String::from("hello");
    // s.push_str(", world! and world is pushed");
    //
    // println!("{}", s);
    //
    // // let mut  st2 = "hello world!";
    // // st2 = "这个是什么";
    // // //这会报错
    // // st2.push_str("这个字面量么");
    //
    // // println!("{}", st2);
    //
    // let x = 5;
    // let y = x;
    // println!("{},{}", x, y);
    //
    // let s1 = String::from("hello");
    // let s2 = s1;
    //
    // //这里s1内存已经释放，访问不到了
    // // println!("s1 = {}, s2 = {}", s1, s2);
    // println!("{}", s2);
    //
    // let s3 = String::from("这是一段文本");
    // let mut s4 = s3.clone();
    // s4.push_str("，是被clone过来的");
    // println!("s3={}, s4={}", s3, s4);
    //
    // owner_fn();
    //
    // let test_str = String::from("hello world");
    // let res = first_word(&test_str);
    // // test_str.clear();
    // println!("res={}", res);


    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

   assert_eq!(slice, [2, 3]);
    assert_ne!(slice, [2, 3]);
}

// 所有权与函数
fn owner_fn() {
    println!("owner_fn函数开始执行。。。");

    let s = String::from("hello");
    takes_ownership(s);
    //这里已经访问不到s的值了，s被移动过
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s2 = String::from("这是一个字符串");

    let s3 = takes_and_gives_back(s2);
    println!("s3:{}", s3);

    let s4 = String::from("个人英雄主义");
    let len = calculate_length(&s4);
    println!("The length of '{}' is {}.", s4, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
// takes_and_gives_back 将传入字符串并返回该值, 移交所有权
fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    // 无法借用不可变局部变量 `s` 作为可变变量
    // s.push_str(" world!");
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
