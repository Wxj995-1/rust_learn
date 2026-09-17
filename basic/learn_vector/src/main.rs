fn main() {
    // ==========1、创建空的 vector: Vec<T> ==========
    let mut v1: Vec<i32> = Vec::new();
    // 空向量，此时没有分配堆内存，push的时候才分配
    v1.push(10);
    v1.push(20);

    // ==========2、创建包含初始值的 vector ==========
    // vec!宏，最常用
    let v2 = vec![1,2,3,4,5];

    // ==========3、丢弃vector ==========
    // vector离开作用域，自动调用drop，释放堆内存
    {
        let v_temp = vec![9,8,7];
        println!("{:?}", v_temp);
    } // 到这里，v_temp销毁，内存释放   

    // ==========4、读取元素：两种方式 ==========
    // 方式1：下标索引，越界直接panic崩溃
    let num = &v2[2];
    println!("下标读取：{}", num);

    // 方式2：.get()，返回Option<&T>，安全，越界返回None（面试重点）
    match v2.get(1) {
        Some(val) => println!("get读取：{}", val),
        None => println!("下标越界"),
    }

    // ==========5、遍历vector ==========
    // 不可变遍历，获取引用
    for item in &v2 {
        println!("item = {}", item);
    }

    // 可变遍历，修改元素
    let mut v3 = vec![100,200,300];
    for item in &mut v3 {
        *item += 1; // 解引用修改
    }
    println!("修改后：{:?}", v3);

    // ==========6、vector使用枚举（Vec存不同类型，只能用枚举） ==========
    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // 同一个Vec里面存放不同类型，只能包装成枚举变体
    let row = vec![
        Cell::Int(10),
        Cell::Float(3.14),
        Cell::Text(String::from("rust")),
    ];
    println!("{:?}", row);

    println!("Hello, world!");
}
