
use std::{ops::{AddAssign, MulAssign, DivAssign, Add, Mul, Div, Sub}, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub fn vec3_(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3{x, y, z}
}

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    // listing 5
    pub fn dot(&self, rhs: &Self)-> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self{
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3{x: 0., y: 0., z: 0.}
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}


impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}


impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}


impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3{x: self.x * rhs,
             y: self.y * rhs,
             z: self.z * rhs}
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3{x: self * rhs.x,
             y: self * rhs.y,
             z: self * rhs.z}
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3{x: self * rhs.x,
             y: self * rhs.y,
             z: self * rhs.z}
    }
}


impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3{x: self.x / rhs,
             y: self.y / rhs,
             z: self.z / rhs}
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3{x: self.x / rhs,
             y: self.y / rhs,
             z: self.z / rhs}
    }
}



impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64)  {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, rhs: &Self) -> bool {
        (self.x == rhs.x) && (self.y == rhs.y) && (self.z == rhs.z)
    }
}

impl Eq for Vec3 {}


impl Vec3 {
    pub fn unit_vector(&self) -> Vec3 {
        return self.clone() / self.length()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_assign() {
        let mut v1 = vec3_(1., 2., 3.);
        let v2 = vec3_(4., 3., 2.);
        let expect = vec3_(5., 5., 5.);
        v1 += &v2;
        assert_eq!(v1, expect);
    }

    #[test]
    fn test_length_squared() {
        let v1 = vec3_(1., 2., 3.);
        assert_eq!(v1.length_squared(), 14.0);
    }

    #[test]
    fn test_length() {
        let v1 = vec3_(3., 4., 0.);
        assert_eq!(v1.length(), 5.0 );
    }

    #[test]
    fn test_cross() {
        let ux = vec3_(1.0, 0., 0.);
        let uy = vec3_(0., 1.0, 0.);
        let uz = vec3_(0., 0., 1.);
        assert_eq!(ux.cross(&uy), uz);
        assert_eq!(uy.cross(&uz), ux);
        assert_eq!(uz.cross(&ux), uy);
    }

}

pub type Point3 = Vec3;