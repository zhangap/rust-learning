fn main() {

    let mut user = User {
        active: true,
        username: String:: from("张无忌"),
        email: String:: from("11@qq.com"),
        address: String::from("武当山")
    };

    println!("{}", user.username);

    user.address = String::from("浪迹天涯");

    let width = 10;
    let height = 5;
    println!("矩形的面积是:{}",area(width, height));

    //使用结构体来重构
    let mut rect = Rectangle {
        width: 10,
        height: 5
    };
    //`Rectangle` cannot be formatted with the default formatter
    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);
    println!("矩形的面积是:{}",area2(rect));

    let rect2 = Rectangle{
        width: 100,
        height: 50
    };

    println!(
        "The rect2 of the rectangle is {} square pixels.",
        rect2.calc_area()
    );

    let rect3 =  Rectangle{
        width: 5,
        height: 4
    };
    let rect4 = Rectangle{
        width: 3,
        height: 2
    };
    println!("The rect3 of the rectangle is {:#?}", rect3.can_hold(&rect4));
}


struct User {
    active: bool,
    username: String,
    email: String,
    address: String
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn calc_area(&self) -> u32 {
        self.width * self.height
    }
    // 判断是否能够覆盖的住
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//计算一个矩形的面积
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}