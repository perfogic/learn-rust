mod string;
mod vec;

use string::{print_string_with_borrow, print_string_normaly};
use vec::{largest};

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
}

fn main() {
    // review_string_learner();
    // review_integer_float_learner();
    review_vec();
}