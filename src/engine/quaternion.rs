use num_traits::FromPrimitive;

use crate::engine::vector3::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion<T>{
    pub x : T,
    pub y : T,
    pub z : T,
    pub w : T,
}

impl<T> Quaternion<T>
where
    T: std::ops::Add<T, Output = T>,
    T: std::ops::Sub<T, Output = T>,
    T: std::ops::Mul<T, Output = T>,
    T: std::ops::Div<T, Output = T>,
    T: num_traits::ToPrimitive + num_traits::FromPrimitive,
    T: Copy,
{

    pub fn new(x: T, y: T, z: T, w: T) -> Self{
       Quaternion{x, y, z, w}
   }

    pub fn length(&self) -> T{
        num_traits::FromPrimitive::from_f64(
            (((self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
                .to_f64().unwrap()).sqrt())).unwrap()
    }

    pub fn normalize(&mut self) -> &mut Self{
        let l = self.length();
        self.x = self.x / l;
        self.y = self.y / l;
        self.z = self.z / l;
        self.w = self.w / l;
        self
    }

    pub fn conjugate(&self) -> Self{
        Quaternion::new(-self.x, -self.y, -self.z, self.w)
    }
}


// mul method
impl<T> std::ops::Mul<Quaternion<T>> for Quaternion<T>
    where
        T: std::ops::Mul<T, Output = T>,
        T: Copy

{
    type Output = Quaternion<T>;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Quaternion{
            x : self.w * rhs.x + self.y * rhs.z - self.z * rhs.y,
            y : self.w * rhs.y + self.z * rhs.x - self.x * rhs.z,
            z : self.w * rhs.z + self.x * rhs.y - self.y * rhs.x,
            w : -self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        }
    }
}

impl<T> std::ops::Mul<Quaternion<T>> for Quaternion<T>
    where
        T: std::ops::Mul<T, Output = T>,
        T: Copy

{
    type Output = Quaternion<T>;

    fn mul(self, rhs: Quaternion<T>) -> Self::Output {
        Quaternion{x : self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y,
                   y : self.y * rhs.w + self.w * rhs.y + self.z * rhs.x - self.x * rhs.z,
                   z : self.z * rhs.w + self.w * rhs.z + self.x * rhs.y - self.y * rhs.x,
                   w : self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z}
    }
}
