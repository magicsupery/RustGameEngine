extern crate num_traits;

use std::fmt::{Formatter, Display};
use num_traits::FromPrimitive;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Vector2<T>
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
    where
        T: std::ops::Mul<T, Output = T>,
        T: std::ops::Add<T, Output = T>,
        T: std::ops::Div<T, Output = T>,
        T: num_traits::ToPrimitive + num_traits::FromPrimitive,
        T: Copy,
{
    pub fn new(x: T, y: T) -> Self{
        Vector2 { x, y}
    }
    pub fn length(&self) -> T
    {
        num_traits::FromPrimitive::from_f64(
            (((self.x * self.x + self.y * self.y).to_f64().unwrap()).sqrt())).unwrap()
    }

    pub fn dot(&self, other: &Self) -> T{
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(&mut self) -> &mut Self{
        let l = self.length();
        self.x = self.x / l;
        self.y = self.y / l;
        self

    }

    pub fn rotate(angle: f64) -> Self{
        let radians = angle.to_radians();
        let sin = radians.sin();
        let cos = radians.cos();

        Vector2{x: x * cos - y * sin, y: x * sin + y * cos}
    }
}

// display
impl<T> std::fmt::Display for Vector2<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// add method
impl<T> std::ops::Add<T> for Vector2<T>
    where
        T: std::ops::Add<T, Output = T>,
        T: Copy,

{
    type Output = Vector2<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vector2{x : self.x + rhs, y : self.y + rhs}
    }
}
impl<T> std::ops::Add<Vector2<T>> for Vector2<T>
where
    T: std::ops::Add<T, Output = T>,

{
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Vector2{x : self.x + rhs.x, y : self.y + rhs.y}
    }
}

// sub method
impl<T> std::ops::Sub<T> for Vector2<T>
    where
        T: std::ops::Sub<T, Output = T>,
        T: Copy,

{
    type Output = Vector2<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vector2{x : self.x - rhs, y : self.y - rhs}
    }
}
impl<T> std::ops::Sub<Vector2<T>> for Vector2<T>
    where
        T: std::ops::Sub<T, Output = T>,

{
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Vector2{x : self.x - rhs.x, y : self.y - rhs.y}
    }
}


// mul method
impl<T> std::ops::Mul<T> for Vector2<T>
    where
        T: std::ops::Mul<T, Output = T>,
        T: Copy,

{
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2{x : self.x * rhs, y : self.y * rhs}
    }
}

impl<T> std::ops::Mul<Vector2<T>> for Vector2<T>
    where
        T: std::ops::Mul<T, Output = T>,

{
    type Output = Vector2<T>;

    fn mul(self, rhs: Vector2<T>) -> Self::Output {
        Vector2{x : self.x * rhs.x, y : self.y * rhs.y}
    }
}


// div method
impl<T> std::ops::Div<T> for Vector2<T>
    where
        T: std::ops::Div<T, Output = T>,
        T: Copy,

{
    type Output = Vector2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector2{x : self.x / rhs, y : self.y / rhs}
    }
}

impl<T> std::ops::Div<Vector2<T>> for Vector2<T>
    where
        T: std::ops::Div<T, Output = T>,

{
    type Output = Vector2<T>;

    fn div(self, rhs: Vector2<T>) -> Self::Output {
        Vector2{x : self.x / rhs.x, y : self.y / rhs.y}
    }
}
