
use std::f64::{INFINITY};
use std::f64::consts::{PI};
use std::rc::Rc;

pub type Shared<T> = Rc<T>;

pub const inf: f64 = INFINITY;
pub const pi: f64 = PI;
pub const rads_per_deg: f64 = pi/ 180.0;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * rads_per_deg
}