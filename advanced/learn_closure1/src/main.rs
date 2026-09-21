


fn main() {
    // 闭包语法：|参数| { 代码 }
    let add = |a: i32, b: i32| -> i32 { a + b };
    // 类型可以省略，Rust自动推断（闭包类型推断能力是fn没有的）
    let add_short = |a,b| a + b;

    println!("{}", add(1,2));
    println!("{}", add_short(3,4));

    println!("Hello, world!");
}
