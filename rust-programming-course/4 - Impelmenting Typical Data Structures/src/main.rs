use std::cell::RefCell; 
use std::rc::{Rc, Weak}; 
#[derive(Debug)] 
struct Node {
    next: Option<Weak<RefCell<Node>>>, 
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}
fn main() {    
    let a = Rc::new(RefCell::new(Node {next: None} )); 
    println!("a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a)); 

    let b = Rc::new(RefCell::new(Node{next: Some(Rc::downgrade(&a))})); 
    println!("B is created: \n a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));  
    println!("b strong count: {:?}, b weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));

    let c = Rc::new(RefCell::new(Node {next: Some(Rc::downgrade(&b))})); 

    (*a).borrow_mut().next = Some(Rc::downgrade(&c)); 

    println!("After creating cycle: \n a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a)); 
    println!("b strong count: {:?}, b weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b)); 
    println!("c strong count: {:?}, c weak count: {:?}", Rc::strong_count(&c), Rc::weak_count(&c)); 

    println!("a {:?}", a);


}