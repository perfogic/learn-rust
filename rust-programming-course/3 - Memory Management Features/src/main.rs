// -------------------------------------------
// 		RefCell Smart Pointer
// -------------------------------------------

use std::{cell::RefCell, rc::Rc};

fn main() {
    /**
     * With the RefCell, you can both borrow borrow_mut and borrow_immutatble at the same time
     * Even you make the collision range between mutable, immutable, the compiler does not show
     * the error, but it will throw an error at the runtime. This will be so useful in doubly linkedlist,
     * and great to combine with Rc, to have multiple reference and each have a mutable capability
     */
    // Example 1: Borrowing rules at run time
    // let mut x = 50;
    // let x1 = &x;
    // let x2 = &x;
    // let x3 = &mut x;

    // println!("{} {} ", x1, x2);

    let a = RefCell::new(10);

    //{ // add the scope later on
    let b = a.borrow();
    let c = a.borrow();
    //}

    drop(b); // add later on, remove after adding the scope above
    drop(c); // add later on
    let d = a.borrow_mut();
    // drop(d) // add later on
    //println!("{} {}", b, c); // later on delete this
    //println!("Value of a is : {:?}", a); // add later on


    // Example 2: Interior mutability
    // let x = 32;
    // let x1 = &mut x;

    let a = RefCell::new(10);
    //let c = *a; // add later on
    let mut b = a.borrow_mut();
    *b = 15;

    drop(b); // add later on
    // If the drop(b) is commented, then we can't not see the value a
    // because it is already borrowed
    println!("{:?}", a);

    // Example 3 
    let a = Rc::new(RefCell::new(String::from("c++")));
    let b = Rc::clone(&a);
    // With this case, because we have multiple reference
    // so we can still see the value of a
    *b.borrow_mut() = String::from("rust");
    println!("{:?}", a);
}
