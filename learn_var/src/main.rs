const MAX_POINT: u32 = 100000;
fn main() {
    //1、变量定义 没有加mut 那么是不可变的
    //let name: type = x;
    let a = 1;
    let mut b: u32 = 1;
    println!("a = {}",a); 
    println!("b = {}",b); 
    b = 2;
    println!("b = {}",b); 
    println!("Hello, world!");

    //2、隐藏性
    let b: f32 = 1.1;
    println!("b = {}",b); 

    //3、常量
    println!("MAX_POINT = {}",MAX_POINT); 

}
