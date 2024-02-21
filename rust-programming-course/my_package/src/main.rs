// crate root for binary crate
use my_package::{Product, Customer, Order, Category};
use array_tool::vec::*;

fn main() {
    let product1 = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
    let product2 = Product::new(2, String::from("Trading book"), 20.0, Category::Books);
    let product3 = Product::new(3, String::from("T-shirt"), 30.0, Category::Clothing);
    
    let customer = Customer::new(1, String::from("Dang"), String::from("longdieu12x@gmail.com"));
    let order = Order::new(1, product1.clone(), customer, 1);

    println!("Order: {:?}", order);

    let set1 = vec![&product1, &product2];
    let set2 = vec![&product2, &product3];
    let intersection = set1.intersect(set2);
    println!("The intersection is {:?}", intersection);
}
