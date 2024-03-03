//Problem 2: We want to change the linked list implementation by making the element part of 
// the node as generic rather then concrete i32. Make approperiate changes to the code below. 
// For printing, T should have the trait bound of  std::fmt::Debug and 
// for the peek to work, T must also have the trait bound of std::marker::Copy  

use std::fmt::Debug;


#[derive(Debug)]
struct Linklist<T> {  // This line needs a fix 
    head: pointer<T>, // This line needs a fix 
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: pointer<T>,  // This line needs a fix  
}
type pointer<T> = Option<Box<Node<T>>>;  // This line needs a fix 

impl<T: Debug + Copy> Linklist<T> {  // This line needs a fix 
    fn new() -> Linklist<T> { // This line needs a fix 
        Linklist { head: None }
    }

    fn add(&mut self, element: T) { // This line needs a fix 
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<T> {  // This line needs a fix 
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<T> {  // This line needs a fix 
        match &self.head {
            Some(H) => Some(H.element),
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none() {
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }
}
fn main() {
    let mut list = Linklist::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);

    println!("{:?}", list.peek());
}
