use std::ops::Deref;

struct Mybox<T>(T);

impl<T> Mybox<T> {
    fn new(x: T) -> Mybox<T> {
        Mybox(x)
    }
}

impl<T> Deref for Mybox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

/*
知识点：解引用强制转换（Deref Coercion）
实现 Deref 更大的作用不是 *，而是自动隐式转换：当需要 &T 的地方传入 &Mybox<T>，编译器会自动调用 deref() 转成 &T。
fn hello(name: &str) {
    println!("你好，{name}");
}

let m = Mybox::new(String::from("Rust"));
hello(&m);   // &Mybox<String> → &String → &str，连续解引用强制转换
过程：&Mybox<String> --deref--> &String --deref--> &str。String 本身实现了 Deref<Target=str>，所以能一路转下去。
这解释了为什么 &String 能当 &str 用——智能指针的"透明性"。
*/
fn hello(name: &str)
{
    println!("hello = {}",name);
}

// 解引用多态与可变性交互
// (1) 当T : Deref<Target = U>时，从&T到&U
// (2) 当T : DerefMut<Target = U>时，从&mut T到&mut U
// (3) 当T : Deref<Target = U>时，从&mut T到&U


fn main() {
    let x = 5;
    let y = Mybox::new(x);
    assert_eq!(5, *y);

    let m = Mybox::new(String::from("Rust"));
    hello(&m);
    println!("Hello, world!");
}