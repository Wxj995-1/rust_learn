use std::collections::HashMap;

fn main() {
    // ========== 2、创建HashMap ==========
    let mut map = HashMap::new();
    // 插入数据：insert(key, value)
    map.insert("name", "wangcai");
    map.insert("age", "10");

    let keys = vec![String::from("blue"),String::from("red")];
    let values = vec![999, 9999];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

 
    // ==========3、读取数据 ==========
    // 方式1: get() 返回 Option<&V>，最安全，推荐
    match map.get("name") {
        Some(val) => println!("name = {}", val),
        None => println!("找不到这个key"),
    }

    // 必须传入 &String
    let target = String::from("blue");
    if let Some(v) = scores.get(&target){
        println!("v = {}",v);
    }


    // 方式2: 下标 []，如果key不存在 → 程序直接panic，生产代码慎用
    println!("age = {}", map["age"]);

    // ==========4、遍历(任意顺序)==========
    println!("=== 遍历所有键值对 ===");
    for (k, v) in &map {
        println!("key:{} , value:{}", k, v);
    }

    // ==========5、更新 ==========
    // 5.1 insert：key存在就覆盖旧值；不存在就新增
    map.insert("age", "1888"); // 覆盖原来的10
    println!("更新后 age = {}", map["age"]);

    // 5.2 entry API：非常高频！不存在才插入，存在不修改
    map.entry("gender").or_insert("male");
    // entry("age")已经存在，不会修改
    map.entry("age").or_insert("99");
    println!("gender = {}", map["gender"]);

    // 5.3 删除
    map.remove("name");
    println!("{:#?}", map);

    // 5.4 添加
    map.entry("name").or_insert("xiaoming");
    println!("{:#?}", map);

    // 5.5 重复添加
    map.entry("name").or_insert("xiaoming");
    println!("{:#?}", map);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace()
    {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map = {:?}",map);

}
