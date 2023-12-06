use std::ops::{Add, Mul, Sub};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Point1D<T> {
    x: T
}

impl<T> Point1D<T> {
    pub fn get(&self) -> &T {
        &self.x
    }

    pub fn new(x: T) -> Point1D<T> {
        Point1D { x }
    }
}

#[derive(Debug)]
pub struct Point2D<T> {
    x: T,
    y: T,
}

impl<T> Point2D<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    fn calculate_sum_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D { x, y }
    }
}

impl<T> Point2D<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
    T: Into<f64>,
{
    pub fn distance_from_origin(&self) -> f64 {
        let val_f64: f64 = self.calculate_sum_squared().into();
        val_f64.sqrt()
    }
}

impl<T> PartialOrd for Point2D<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.calculate_sum_squared().partial_cmp(&other.calculate_sum_squared())?)
    }
}

impl<T> Ord for Point2D<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.calculate_sum_squared().cmp(&other.calculate_sum_squared())
    }
}

impl<T> Eq for Point2D<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
    T: PartialEq,
{
}

impl<T> PartialEq for Point2D<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.calculate_sum_squared() == other.calculate_sum_squared()
    }
}

impl<T> Copy for Point2D<T> where T: Copy { }

impl<T> Clone for Point2D<T>
where
    T: Clone,
{
    fn clone(&self) -> Point2D<T> {
        Point2D {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}