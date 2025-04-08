mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant2() {
    // 绝对路径使用
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径使用
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {
    println!("serve_order");
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        crate::serve_order();
    }
    fn cook_order() {
        println!("cook_order");
    }
    //早餐
    pub struct Breakfast {
        // 面包
        pub toast: String,
        // 时令水果
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                // 桃子
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    //私有属性不能修改，以下代码会报错
    // meal.seasonal_fruit = String::from("blueberries");
}
