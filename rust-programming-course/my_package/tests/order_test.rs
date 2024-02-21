use my_package::{Category, Customer, Order, Product};
mod helpers;

#[test]
fn test_total_bill_without_discount() {
    helpers::common_test();
    let product = Product::new(1, String::from("Macbook"), 1000.00, Category::Electronics);
    let customer = Customer::new(1, String::from("Dang"), String::from("test@gmail.com"));
    let order = Order::new(1, product, customer, 3);
    assert_eq!(format!("{:.3}", order.total_bill()), "3300.000");
}

#[test]
fn test_total_bill_with_discount() {
    let product = Product::new(1, String::from("Macbook"), 1000.00, Category::Electronics);
    let customer = Customer::new(1, String::from("Dang"), String::from("test@gmail.com"));
    let order = Order::new(1, product, customer, 6);
    assert_eq!(format!("{:.3}", order.total_bill()), "6599.900");
}