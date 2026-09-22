enum List
{
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons,Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5,Rc::new(Cons(20,Rc::new(Nil)))));

    println!("count after creating a = {}",Rc::strong_count(&a));

    let b = Cons(3,Rc::clone(&a));
    println!("count after bind to b = {}",Rc::strong_count(&a));

    {
        let c = Cons(3,Rc::clone(&a));
        println!("count after bind to c = {}",Rc::strong_count(&a));

    }
    println!("count at end = {}",Rc::strong_count(&a));

    println!("Hello, world!");
}
