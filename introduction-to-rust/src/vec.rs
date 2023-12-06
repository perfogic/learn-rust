use std::cmp::PartialOrd;

pub fn largest(arr: &[i32]) -> i32 {
    let mut largest = arr[0];
    for &item in arr {
        if largest < item {
            largest = item;
        }
    }
    largest
}

pub fn genericLargest<T: PartialOrd + Copy>(arr: &[T]) -> T {
    let mut largest = arr[0];
    for &item in arr {
        if largest < item {
            largest = item;
        }
    }
    largest
}