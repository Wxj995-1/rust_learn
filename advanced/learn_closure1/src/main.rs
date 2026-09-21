
/* 
fn main() {
    // 闭包语法：|参数| { 代码 }
    let add = |a: i32, b: i32| -> i32 { a + b };
    // 类型可以省略，Rust自动推断（闭包类型推断能力是fn没有的）
    let add_short = |a,b| a + b;

    println!("{}", add(1,2));
    println!("{}", add_short(3,4));

    println!("Hello, world!");
}
*/

fn main()
{
    let use_closure = || {
        println!("this is a closure");
    };
    use_closure();
    println!("hello world!");

    // 闭包定义会为每个参数和返回值类型推导一个具体的类型 ，但是不能推导两次
    let add_one_v2 = |x:u32|  -> u32{ x + 1};
    let add_one_v3 = |x| {x + 1};  
    let add_one_v4 = |x| x + 1;

    let a = add_one_v1(5);
    let b = add_one_v2(5);
    let c = add_one_v3(5);
    let d = add_one_v4(5);
    println!("a = {},b = {},c = {},d = {}",a,b,c,d);

    // 不能推导两次的例子
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    println!("s = {}", s);

    // 只允许推导为第一次推导的类型
    let n =example_closure(5.to_string());
    println!("s = {}",n);

    // 捕捉环境中的变量
    let i = 1;
    let exe  = |x| x+i;
    let r = exe(5);
    println!("r = {}",r);
}

// 语法格式
fn add_one_v1(x: u32) -> u32{
    x + 1
}




