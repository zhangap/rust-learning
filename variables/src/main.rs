use std::io;
fn main() {
    test_fn();
    test_fn1();
    test_fn2();
    num_operations();
    compound_types_fn();
    visit_array_index();
    test_fn_expression();
}

fn test_fn() {
    let mut x = 5;
    println!("The value of x is:{}", x);
    x = 6;
    println!("The value of x is:{}", x);
    //定义常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("定义的常量值:{}", THREE_HOURS_IN_SECONDS);

    //变量的遮蔽或隐藏
    let name = "张三丰";
    let name = "武当".to_owned() + name;

    println!("The value of name is:{}", name);
}

fn test_fn1() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn test_fn2() {
    let x = 2.0;
    let y: f32 = 3.0;

    let t = true;
    let f: bool = false;
    println!("The value of y is: {t}, {f}");
}

fn num_operations() {
    let sum = 5 + 10;
    println!("Sum is:{}", sum);
    let difference = 95.5 - 4.3;
    println!("Difference is:{}", difference);

    let product = 4 * 30;
    println!("Product is:{}", product);

    let quotient = 56.7 / 32.2;
    println!("quotient is:{}", quotient);
    let remainder = 43 % 5;
    println!("remainder is:{}", remainder);
    let truncated = -5 / 3; // 结果为 -1
    println!("truncated is:{}", truncated);
}

// 元组类型
fn compound_types_fn() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 解构
    let (_x, y, _z) = tup;
    println!("The value of y is: {_x}, {y}, {_z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred:{five_hundred},six_point_four:{six_point_four},one:{one}");

    // 数组类型
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let arr: [&str; 3] = ["张无忌", "张翠山", "金毛狮王"];

    let arr2 = ["zap"; 5];
    println!("arr2:{arr2:?}");
    println!("数组的第二个元素是：{}", arr2[1]);
}

//访问数组下标
fn visit_array_index() {
    let arr = [
        "张三丰",
        "张无忌",
        "张翠山",
        "金毛狮王",
        "元贞",
        "赵敏",
        "蛛儿",
    ];

    println!("请选择你喜欢的人物ID");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index.trim().parse().expect("请输入一个数字");
    println!("你最喜欢的人物是：{}", arr[index]);
}

fn test_fn_expression() {
    //注意表达式的结尾没有分号，有分号表示是语句
    let y = {
        let x = 5;
        x + 10
    };

    println!("The value of y is:{y}");
}

fn five() -> i32 {
    5
    // return 5;
}
