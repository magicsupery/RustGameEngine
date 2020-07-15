extern crate num_traits;

use self::num_traits::FromPrimitive;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4<T>{
    pub m : [[T; 4]; 4],
}

impl<T> Matrix4<T>
where
    T: num_traits::FromPrimitive + Copy,
{
    pub fn new() -> Self{
        Matrix4{ m: [[num_traits::FromPrimitive::from_u32(0).unwrap(); 4]; 4] }
    }
}

// display
impl<T> Display for Matrix4<T>
    where
        T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:<8} {:<8} {:<8} {:<8}]\n[{:<8} {:<8} {:<8} {:<8}]\n[{:<8} {:<8} {:<8} {:<8}]\n[{:<8} {:<8} {:<8} {:<8}]",
            self.m[0][0], self.m[0][1], self.m[0][2], self.m[0][3],
            self.m[1][0], self.m[1][1], self.m[1][2], self.m[1][3],
            self.m[2][0], self.m[2][1], self.m[2][2], self.m[2][3],
            self.m[3][0], self.m[3][1], self.m[3][2], self.m[3][3])
    }
}

// mul method
// impl<T> std::ops::Mul<T> for Vector2<T>
//     where
//         T: std::ops::Mul<T, Output = T>,
//         T: Copy,
//
// {
//     type Output = Vector2<T>;
//
//     fn mul(self, rhs: T) -> Self::Output {
//         Vector2{x : self.x * rhs, y : self.y * rhs}
//     }
// }

impl<T> std::ops::Mul<Matrix4<T>> for Matrix4<T>
    where
        T: std::ops::Add<T, Output = T>,
        T: std::ops::Mul<T, Output = T>,
        T: num_traits::FromPrimitive + Copy,
{
    type Output = Matrix4<T>;

    fn mul(self, rhs: Matrix4<T>) -> Self::Output {

        let mut result = Matrix4{ m: [[num_traits::FromPrimitive::from_u32(0).unwrap(); 4]; 4] };
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[i][0] + rhs.m[0][j] +
                    self.m[i][1] + rhs.m[1][j] +
                    self.m[i][2] + rhs.m[2][j] +
                    self.m[i][3] + rhs.m[3][j]
            }
        }
        result
    }
}


