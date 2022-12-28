
use std::f64::{INFINITY};
use std::f64::consts::{PI};
use std::rc::Rc;

use rand::{rngs::ThreadRng, Rng};


pub type Shared<T> = Rc<T>;

pub const INF: f64 = INFINITY;
pub const RADS_PER_DEG: f64 = PI/ 180.0;


#[inline(always)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * RADS_PER_DEG
}

pub fn random_unif_1(rng: &mut ThreadRng) -> f64 {
    return rng.gen::<f64>()
}

pub fn random_unif(rng: &mut ThreadRng, min: f64, max: f64) -> f64 {
    return min + (max - min) * random_unif_1(rng)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}