#[derive(Debug)]
enum List{
    Cons(i32,RefCell<Rc<List>>),
    Nil,
}

impl List{
    fn tail(&self)-> Option<&RefCell<Rc<List>>>
    {
        match self // self = &List 
        {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons,Nil};


fn main() {
    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a 的强引用计数 = {}", Rc::strong_count(&a)); // 1

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a 的强引用计数 = {}", Rc::strong_count(&a)); // 2
    println!("b 的强引用计数 = {}", Rc::strong_count(&b)); // 1

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);   // 把 a 的尾巴从 Nil 改成 b
    }

    println!("a 的强引用计数 = {}", Rc::strong_count(&a)); // 2
    println!("b 的强引用计数 = {}", Rc::strong_count(&b)); // 2

    println!("Hello, world!");
}
     