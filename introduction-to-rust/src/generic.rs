use std::ops::{Add, Mul, Sub};

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
    y: T
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D { x, y }
    } 
}

impl<T> Point2D<T> where 
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy
{
    pub fn distance_from_origin(&self) -> f64
    where
        T: Into<f64>,
    {
        let x_f64: f64 = self.x.into();
        let y_f64: f64 = self.y.into();
        (x_f64 * x_f64 + y_f64 * y_f64).sqrt()
    }
}