use std::rc::Rc;
enum List {
    Cons(i32, Option<Rc<List>>),
}
fn main() {
    /**
     * Here, both a,b,c are owner of the data List::Cons(1, Some(Rc::new(List::Cons(2, None))))
     * Which is done by using Rc smart pointer, the Rc::clone is not clone a deep copy of
     * the data like other clone, it just add a reference count += 1 (which is really effienct),
     * strong_count() here show all the reference count which will illustrate you about this
     * concept. Author: Perfogic!
     */
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None)))));
    println!("Reference count after a: {}", Rc::strong_count(&a));
    {
        let b = List::Cons(3, Some(Rc::clone(&a)));
        println!("Reference count after b: {}", Rc::strong_count(&a));

        let c = List::Cons(4, Some(Rc::clone(&a)));
        println!("Reference count after c: {}", Rc::strong_count(&a));
    }
    println!("Reference count after scope: {}", Rc::strong_count(&a));
}
