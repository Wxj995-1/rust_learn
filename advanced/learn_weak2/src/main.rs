// Rc 只能向下共享，但真实树需要"子节点能回头找到父节点"。父持有子用 Rc，子指向父用 Weak（否则父子互指又成环）。
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,        // 指向父：弱，避免环
    children: RefCell<Vec<Rc<Node>>>,   // 持有子：强
}


fn main() {
    let leaf = Rc::new(Node{value: 3,parent: RefCell::new(Weak::new()),children: RefCell::new(vec![])});
    println!("leaf strong = {}, weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));

    let branch = Rc::new(Node{value: 5, parent: RefCell::new(Weak::new()),children: RefCell::new(vec![Rc::clone(&leaf)])});
    println!("leaf strong = {}, weak = {}",Rc::strong_count(&branch),Rc::weak_count(&branch));

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf strong = {}, weak = {}",Rc::strong_count(&branch),Rc::weak_count(&branch));
    println!("leaf strong = {}, weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));

    println!("Hello, world!");
}
