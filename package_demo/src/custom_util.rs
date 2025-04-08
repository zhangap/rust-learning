pub mod token_util {
    pub fn get_token() -> String {
        String::from("auth1234556")
    }
    pub fn set_token(token_value:String) {
        let token = token_value;
        println!("要设置的token值是：{}", token);
        // token = token_value;
    }
}





