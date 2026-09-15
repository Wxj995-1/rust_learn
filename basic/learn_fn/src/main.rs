fn other_fun() {
    println!("this is a function");
}

fn other_fun1(a: i32, b: u32) {
    println!("a = {}, b = {}", a, b);
}

fn other_fun2(a: u32, b: u32) -> u32 {
    // let result = a + b as i32;
    // result
    a + b
}


fn main() {
    other_fun();
    let a: i32 = -1;
    let b: u32 = 2;
    other_fun1(a, b);
    let c: u32 = 9;
    let r: u32 = other_fun2(b, c);
    println!("r = {}", r);

    println!("Hello, world!");
}
