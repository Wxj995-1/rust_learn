//fn bar() -> !
//{
//    loop{}
//}
/*
# Rust `!` Never Type（永不返回类型）

`!` 读作 **never type**，中文叫**永不返回类型 / 空类型（empty type）

> 核心定义：**这个类型没有任何有效值。代表代码永远不会走到 “返回” 这一步
*/

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // continue类型 !，自动转为u32，分支直接跳回loop
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

