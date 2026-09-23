// 访问或者修改可变静态变量
/*
static HELLO_WORLD : &str = "hello world";
fn main() {
    println!("{}",HELLO_WORLD);
} */


/*
static mut COUNTER: u32 = 0;
fn add_count(inc: u32)
{
    unsafe{
        COUNTER += inc;
    }
}


fn main()
{
    add_count(3);
    unsafe{
        println!("counter: {}", COUNTER);
    }

} */

use std::ptr::{addr_of, addr_of_mut};

static mut COUNTER: u32 = 0;

fn add_count(inc: u32) {
    unsafe {
        *addr_of_mut!(COUNTER) += inc;
    }
}

fn main() {
    add_count(3);
    unsafe {
        println!("counter: {}", *addr_of!(COUNTER));
    }
}