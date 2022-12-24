use std::{ops::{AddAssign, MulAssign, DivAssign}, fmt::Debug};


#[derive(Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3{x, y, z}
}

impl Vec3 {
    pub fn length_squared(self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        return self.length_squared().sqrt()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3{x: 0., y: 0., z: 0.}
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_assign() {
        let mut v1 = vec3(1., 2., 3.);
        let v2 = vec3(4., 3., 2.);
        let expect = vec3(5., 5., 5.);
        v1 += v2;
        assert_eq!(v1, expect);
    }

    #[test]
    fn test_length_squared() {
        let v1 = vec3(1., 2., 3.);
        assert_eq!(v1.length_squared(), 14.0);
    }

    #[test]
    fn test_length() {
        let v1 = vec3(3., 4., 0.);
        assert_eq!(v1.length(), 5.0 );
    }

}
