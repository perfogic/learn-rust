mod string;
mod vec;
mod generic;
mod traits;

use string::{print_string_with_borrow, print_string_normaly};
use vec::{largest, genericLargest};
use generic::{Point1D, Point2D};
use traits::{Momo, VnPay, Initialize, Bank};

fn review_string_learner() {
    let hello_word: String = String::from("hello world");

    print_string_with_borrow(&"Test");
    print_string_with_borrow(&hello_word);
    print_string_normaly(hello_word);
}

fn review_integer_float_learner() {
    let x: i8 = 5;
    let y: i16 = 125;

    // Here we should use i16::from, to make error if we cast wrong way like cast i8 to i16
    println!("Value plus x + y = {}", i16::from(x) + y);
}

fn review_vec() {
    let number_list = vec![1, 12, 3, 4, 15];

    let large = largest(&number_list);

    println!("Largest value: {}", large);

    let largeGene = genericLargest(&number_list);

    println!("Largest value with generic: {}", largeGene);
}

fn review_generic() {
    let p1D: Point1D<i32> = Point1D::new(5);

    println!("Point 1d: {:?} with x: {}", p1D, p1D.get());

    let p2D = Point2D::new(8, 6);
    println!("Point 2d: {:?} with distance: {}", p2D, p2D.distance_from_origin());

    let listPoints = vec![Point2D::new(8, 6), Point2D::new(3, 4), Point2D::new(16, 18), Point2D::new(20, 5)];
    let largePoint = genericLargest(&listPoints);
    println!("Largest value with generic: {:?}", largePoint);
}

fn review_trait() {
    let momo = Momo::new(1, "Dang".to_string());
    let vn_pay = VnPay::new(1, "Dang".to_string());

    momo.print_default_balance();
    vn_pay.print_default_balance();
}

fn main() {
    // review_string_learner();
    // review_integer_float_learner();
    // review_vec();
    // review_generic();
    review_trait();
}