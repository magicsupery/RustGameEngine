extern crate num_traits;

use std::fmt::{Formatter, Display};
use num_traits::FromPrimitive;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Vector3<T>
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T>
    where
        T: std::ops::Mul<T, Output = T>,
        T: std::ops::Add<T, Output = T>,
        T: std::ops::Div<T, Output = T>,
        T: num_traits::ToPrimitive + num_traits::FromPrimitive,
        T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self{
        Vector3 { x, y, z}
    }
    pub fn length(&self) -> T
    {
        num_traits::FromPrimitive::from_f64(
            (((self.x * self.x + self.y * self.y + self.z * self.z).to_f64().unwrap()).sqrt())).unwrap()
    }

    pub fn dot(&self, other: &Self) -> T{
        self.x * other.x + self.y * other.y + self.z * self.z
    }

    pub fn normalize(&mut self) -> &mut Self{
        let l = self.length();
        self.x = self.x / l;
        self.y = self.y / l;
        self.z = self.z / l;
        self

    }

    pub fn cross(&self, other: &Self) -> Self{
        Vector3{
            x : self.y * other.z - self.z * other.y,
            y : self.z * other.x - self.x * other.z,
            z : self.x * other.y - self.y * other.x,
        }
    }
}

// display
impl<T> std::fmt::Display for Vector3<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// add method
impl<T> std::ops::Add<T> for Vector3<T>
    where
        T: std::ops::Add<T, Output = T>,
        T: Copy,

{
    type Output = Vector3<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vector3{x : self.x + rhs, y : self.y + rhs, z: self.z + rhs}
    }
}
impl<T> std::ops::Add<Vector3<T>> for Vector3<T>
where
    T: std::ops::Add<T, Output = T>,

{
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Vector3{x : self.x + rhs.x, y : self.y + rhs.y, z: self.z + rhs.z}
    }
}

// sub method
impl<T> std::ops::Sub<T> for Vector3<T>
    where
        T: std::ops::Sub<T, Output = T>,
        T: Copy,

{
    type Output = Vector3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vector3{x : self.x - rhs, y : self.y - rhs, z : self.z - rhs}
    }
}
impl<T> std::ops::Sub<Vector3<T>> for Vector3<T>
    where
        T: std::ops::Sub<T, Output = T>,

{
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Vector3{x : self.x - rhs.x, y : self.y - rhs.y, z:self.z - rhs.z}
    }
}


// mul method
impl<T> std::ops::Mul<T> for Vector3<T>
    where
        T: std::ops::Mul<T, Output = T>,
        T: Copy,

{
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3{x : self.x * rhs, y : self.y * rhs, z : self.z * rhs}
    }
}

impl<T> std::ops::Mul<Vector3<T>> for Vector3<T>
    where
        T: std::ops::Mul<T, Output = T>,

{
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Vector3{x : self.x * rhs.x, y : self.y * rhs.y z : self.z * rhs.z}
    }
}


// div method
impl<T> std::ops::Div<T> for Vector3<T>
    where
        T: std::ops::Div<T, Output = T>,
        T: Copy,

{
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector3{x : self.x / rhs, y : self.y / rhs, z : self.z / rhs}
    }
}

impl<T> std::ops::Div<Vector3<T>> for Vector3<T>
    where
        T: std::ops::Div<T, Output = T>,

{
    type Output = Vector3<T>;

    fn div(self, rhs: Vector3<T>) -> Self::Output {
        Vector3{x : self.x / rhs.x, y : self.y / rhs.y, z : self.z / rhs.z}
    }
}
