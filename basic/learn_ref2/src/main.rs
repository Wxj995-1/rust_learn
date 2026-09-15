/*
let s = &s1;        ┌───不可变借用 s 生效
calcute_length(s);  │
println!("len={}",len); └── s最后一次使用，不可变借用【结束】
---------------------------------------
let ms=&mut s1;     ┌──可变借用 ms 生效
modify_s(ms);       └──可变借用结束
println!("s1={}",s1);

*/
fn calcute_length(s:&String)->usize{
    s.len()
}
fn modify_s(s:&mut String){
    s.push_str("_modified");
}
fn main(){
    let mut s1 = String::from("hello");
    // ① 借这本书只读，拿到借书证 s
    let s = &s1;
    // ② 使用这个只读借书证，计算长度
    let len = calcute_length(s);
    // ③ 打印长度。这一行之后，再也没有写 s 了！借书证用完，归还！
    println!("len = {}", len);

    // 此时：只读借书证已经归还！没有人在读这本书了！
    // ④ 现在可以申请【可写借书证】ms
    let ms = &mut s1;
    // ⑤ 修改这本书
    modify_s(ms);
    // 可写借书证用完，归还
    println!("s1 = {}", s1);
}


/*
fn main(){
    let mut s1 = String::from("hello");
    let s = &s1;          // 只读借书证 s
    let len = calcute_length(s);

    let ms = &mut s1;     // 申请可写借书证  ERROR！！
    modify_s(ms);

    println!("len = {}", len); // 这里还要使用s对应的只读数据！
    println!("s1 = {}", s1);
}

*/