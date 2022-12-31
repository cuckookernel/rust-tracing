
use std::{ops::{AddAssign, MulAssign, DivAssign, Add, Mul, Div, Sub, Neg}, fmt::Debug};
use crate::rtweekend::{random_unif, random_unif_1};
use rand::rngs::ThreadRng;


#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub fn vec3_(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3{x, y, z}
}

pub fn color(r: f64, g: f64, b: f64) -> Vec3 {
    Vec3{x:r, y:g, z:b}
}

pub fn point3(x: f64, y: f64, z: f64) -> Vec3 {
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

// component wise multiply: &vec1 * &vec2
impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3{x: self.x * rhs.x,
             y: self.y * rhs.y,
             z: self.z * rhs.z}
    }
}



// scalar multiply &vec * c
impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3{x: self.x * rhs,
             y: self.y * rhs,
             z: self.z * rhs}
    }
}

// scalar multiply: c * &vec
impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3{x: self * rhs.x,
             y: self * rhs.y,
             z: self * rhs.z}
    }
}


// scalar multiply: c * vec
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3{x: self * rhs.x,
             y: self * rhs.y,
             z: self * rhs.z}
    }
}

// scalar divide &vec / c
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

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, rhs: &Self) -> bool {
        (self.x == rhs.x) && (self.y == rhs.y) && (self.z == rhs.z)
    }
}

impl Eq for Vec3 {}


impl Vec3 {
    pub fn unit_vector(&self) -> Self {
        self.clone() / self.length()
    }

    pub fn rand_unif(rng: &mut ThreadRng, min: f64, max: f64) -> Self {
        vec3_(random_unif(rng, min, max),
              random_unif(rng, min, max),
              random_unif(rng, min, max))
    }

    pub fn rand_in_sphere_1(rng: &mut ThreadRng) -> Self {
        loop {
            let vec = vec3_(random_unif_1(rng),
                                  random_unif_1(rng),
                                  random_unif_1(rng));
            if vec.length_squared() >= 1.0 {
                continue
            } else {
                return vec
            }
        }
    }

    pub fn rand_hemisphere(rng: &mut ThreadRng, normal: &Vec3) -> Self {
        // listing 39
        let in_unit_sphere = Vec3::rand_in_sphere_1(rng);

        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_unit_vector(rng: &mut ThreadRng) -> Self {
        Self::rand_in_sphere_1(rng).unit_vector()
    }

    // listing 45
    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        return (self.x.abs() < eps) & (self.y.abs() < eps) & (self.z.abs() < eps)
    }

    // listing 47
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        return self - &(2.0 * self.dot(normal) * normal);
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
pub type Color = Vec3;