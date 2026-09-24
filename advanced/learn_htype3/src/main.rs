// (1) 最典型的就是str

/*
&str 是一个引用（指针）。指针的大小是固定的（和它指向的数据多长无关），所以 &str 是 Sized，能用。
而且 &str 不是普通指针，它是胖指针（fat pointer），包含两部分：
&str = {
    ptr: *const u8,   // 指向文本的第一个字节
    len: usize,       // 文本有多少字节
}
*/
fn main() {
    let s1: &str = "hello";
    let s2: &str = "world";
    println!("Hello, world!");
}
