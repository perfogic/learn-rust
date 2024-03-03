//Problem 3: Change the code below so that the element part of the node is a generic rather than 
// concrete i32 value 
// Solution: 

use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
struct Doubly_Linklist<T> {
    head: pointer<T>,
    tail: pointer<T>,
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: pointer<T>,
    prev: pointer<T>,
}

type pointer<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T: std::fmt::Debug + std::marker::Copy + std::fmt::Display> Doubly_Linklist<T> {
    fn new() -> Self {
        Doubly_Linklist {
            head: None,
            tail: None,
        }
    }

    fn add(&mut self, element: T) {
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }

            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn add_back(&mut self, element: T) {
        let new_tail = Node::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }
    fn remove(&mut self) -> Option<T> {
        if self.head.is_none() {
            println!("List is empty so we can not remove");
            None
        } else {
            let removed_val = self.head.as_ref().unwrap().borrow().element;
            self.head
                .take()
                .map(|old_head| match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail = None;
                        println!("List is empty after removal");
                        None
                    }
                });
            Some(removed_val)
        }
    }

    fn remove_back(&mut self) {
        if self.tail.is_none() {
            println!("list is emtpy so we can not remove");
        } else {
            self.tail
                .take()
                .map(|old_tail| match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                        self.tail.clone()
                    }
                    None => {
                        self.head.take();
                        println!("List is empty after removal");
                        None
                    }
                });
        }
    }

    fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("{}", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
    }
}

impl<T> Node<T> {
    fn new(element: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            element: element,
            next: None,
            prev: None,
        }))
    }
}
fn main() {
    let mut list1 = Doubly_Linklist::new();

    list1.add(30);
    list1.add(32);
    list1.add(34);
    list1.add(36);
    list1.print();

    println!("After removing the elements from the back/end");
    list1.remove_back();
    list1.remove_back();
    list1.print();
}
