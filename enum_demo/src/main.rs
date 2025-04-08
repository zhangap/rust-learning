enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum Option<T> {
//     None,
//     Some(T),
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let four = IpAddrKind::V4;
    route(IpAddrKind::V4);

    let some_number = Some(5);
    let some_char = Some('e');
    // let absent_number: Option<i32> = None;

    let res = value_in_cents(Coin::Penny);
    println!("value_in_cents is {}", res);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five is {}", five.unwrap());
    println!("six is {}", six.unwrap());

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("max is {}", max),
        _ => {}
    }
}

fn route(ip_kind: IpAddrKind) {
    let home = IpAddr {
        kind: ip_kind,
        address: String::from("127.0.0.1"),
    };

    // println!("{}", home);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
