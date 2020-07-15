use num_traits::FromPrimitive;

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
}
