use std::collections::HashMap;
fn main() {
    // 创建一个hashMap
    let mut map = HashMap::new();
    map.insert("张三", 100);
    map.insert("李四", 98);
    map.insert("王五", 99);
    //遍历
    for (k, v) in map {
        println!("{}: {}", k, v);
    }

    custom_create_hash_map();
    hash_map_fn2();
}

//自定义创建map
fn custom_create_hash_map() {
    let teams = vec![
        String::from("Blue"),
        String::from("Yellow"),
        String::from("Red"),
    ];
    let intial_scores = vec![10, 50, 30];
    let mut scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();

    let pinkKey = String::from("Pink");
    scores.insert(&pinkKey, &40);

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }
}

fn  hash_map_fn2() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}", map);
    //这里会报错，如果想要不报错，map.insert(&field_name, &field_value);
    // println!("{},{}", field_name, field_value);

    println!("获取值{}", map.get("Favorite color").unwrap());


    //更新map值entry().or_insert
    let mut map2 = HashMap::new();
    map2.insert(String::from("中国"), "毛主席");
    map2.insert(String::from("美国"), "克林顿");
    map2.entry(String::from("美国")).or_insert("特朗普");
    let res = map2.entry(String::from("德国")).or_insert("撒切尔夫人");

    println!("{:?}", res);
    println!("{:#?}", map2);

    let text = "hello world wonderful world";
    let mut map3 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map3.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map3);




}
