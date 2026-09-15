// ========== 不可变借用 ==========
fn borrow_string(s: &String) {
    println!("不可变借用读取：{}", s);
}

// ========== 可变借用 ==========
fn change_str(s: &mut String) {
    s.push_str(" 追加内容");
    println!("可变借用修改：{}", s);
}

// ========== 返回String，不能返回局部变量引用 ==========
fn return_ownership() -> String {
    let s = String::from("返回所有权");
    s
}

// ========== 计算长度 ================
fn calcute_length(s: &String) ->usize{
    s.len()
}
fn main() {
    // 不可变借用演示
    let s1 = String::from("不可变借用测试");
    borrow_string(&s1);
    borrow_string(&s1);
    println!("s1还能用:{}", s1);

    // 可变借用演示
    let mut s2 = String::from("可变借用");
    change_str(&mut s2);
    println!("main查看s2:{}", s2);

    // 切片 &str
    let s3 = String::from("slice demo");
    let slice = &s3[0..5];
    println!("字符串切片:{}", slice);

    // 返回所有权
    let s4 = return_ownership();
    println!("return s4:{}", s4);

    // 长度测试
    let s5 = String::from("hello world");
    let s6 = &s5;
    let len = calcute_length(&s6);
    println!("s5_len = {}",len);

}
