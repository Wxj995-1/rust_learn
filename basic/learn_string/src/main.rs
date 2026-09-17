fn main() {
    // ==========1、创建一个空String==========
    let mut s1 = String::new();
    s1.push_str("empty string");
    println!("s1: {}", s1);

    // ==========2、通过字面值创建String==========
    // 2.1 String::from() 在堆上分配字符串
    let s2 = String::from("hello rust");
    println!("s2: {}", s2);
    // 2.2 str字面量（&str，硬编码在程序二进制，无所有权）
    let s3 = "hello str";
    println!("s3: {}", s3);

    // ==========3、更新String========== 
    let mut s4 = String::from("hi");
    // 3.1 push_str：追加字符串切片，不获取所有权
    s4.push_str(" world");
    println!("push_str -> {}", s4);
    // 3.2 push：追加单个字符(char)
    s4.push('!');
    println!("push char -> {}", s4);
    // 3.3 + 拼接字符串，注意：+左边会转移所有权
    let s_a = String::from("abc");
    let s_b = String::from("def");
    let s_c = s_a + &s_b; // s_a 所有权被移动，后面不能再用s_a
    println!("+拼接: {}", s_c);
    //3.4 format! 宏拼接，不转移所有权，推荐多字符串拼接
    let s_x = String::from("one");    
    let s_y = String::from("two");
    let s_z = format!("{} + {}", s_x, s_y);
    println!("format拼接: {}", s_z);

    // ==========4、String 索引【重点！Rust不支持直接 s[0]取字符】==========
    // let s_test = String::from("中文");
    // let ch = s_test[0]; // 编译报错！String是UTF-8，字节和字符不是一一对应

    // ==========5、str切片索引（只能取字节切片）==========
    let s_slice = String::from("rust");
    let sub = &s_slice[0..2]; // 取字节[0,2)，得到"ru"
    println!("str切片: {}", sub);
    println!("str.lem = {}", s_slice.len());
    // 注意：如果对中文取字节切片，很容易切到字符中间，运行时panic！

    // ==========6、遍历==========
    let s_loop = String::from("中文a");
    //6.1 chars()：按Unicode字符遍历，最常用
    println!("s_loop.lem = {}", s_loop.len());
    println!("=== chars遍历 ===");
    for c in s_loop.chars() {
        println!("{}", c);
    }
    //6.2 bytes()：按原始字节遍历
    println!("=== bytes遍历 ===");
    for b in s_loop.bytes() {
        println!("{}", b);
    }
}
