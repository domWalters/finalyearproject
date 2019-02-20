use std::{fmt::Debug, marker::{Send, Sync}};
use num::traits::{Num, NumCast, Bounded, ToPrimitive};
use rand::distributions::uniform::SampleUniform;

pub trait DataTrait: Num + NumCast + Bounded + ToPrimitive + PartialOrd + Debug + Copy + Clone + Send + Sync + SampleUniform {
    fn interval(&self, percentage: f64) -> (Self, Self);
}

impl DataTrait for usize {
    fn interval(&self, percentage: f64) -> (usize, usize) {
        let difference = ((*self as f64) * (percentage / 100.0)) as usize;
        (self - difference, self + difference)
    }
}
impl DataTrait for f64 {
    fn interval(&self, percentage: f64) -> (f64, f64) {
        let difference = self * (percentage / 100.0);
        (self - difference, self + difference)
    }
}
