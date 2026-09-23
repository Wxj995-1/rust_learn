use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // Weak::new() 创建的是一个空的弱引用——它不指向任何东西，也不需要指向任何东西
    let a = Rc::new(Cons(5, RefCell::new(Weak::new())));
    println!("1, a strong = {}, weak = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("1, a tail = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Weak::new())));

    // b 的尾巴弱指向 a
    if let Some(link) = b.tail() {
        *link.borrow_mut() = Rc::downgrade(&a);
    }
    println!("2, a strong = {}, weak = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("2, b strong = {}, weak = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("2, b tail = {:?}", b.tail());

    // a 的尾巴弱指向 b → 形成弱环
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }
    println!("3, a strong = {}, weak = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("3, b strong = {}, weak = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("3, a tail = {:?}", a.tail());

    println!("Hello, world!");
}