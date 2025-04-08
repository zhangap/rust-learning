mod custom_util;
fn main() {
    println!("Hello, world!");
    let res = custom_util::token_util::get_token();
    println!("{:?}", res);
}
