fn main() {
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    loop_tag_fn();
    while_loop_fn();
    for_loop_fn();
}

//循环标签
fn loop_tag_fn() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);

        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if (remaining == 9) {
                break;
            }
            if (count == 2) {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

// while循环
fn while_loop_fn() {
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < arr.len() {
        println!("arr[{}] = {}", index, arr[index]);
        index += 1;
    }
}

fn for_loop_fn() {
    for number in (1..=10).rev() {
        println!("{}!", number);
    }
}

//温度转换
fn temperature_conversion(f: i32) -> i32 {
    return 5 * (f - 32) / 9;
}
