use std::{fmt::Debug, marker::{Send, Sync}};
use num::traits::{Num, NumCast, Bounded, ToPrimitive};
use rand::distributions::uniform::SampleUniform;

pub trait DataTrait: Num + NumCast + Bounded + ToPrimitive + PartialOrd + Debug + Copy + Clone + Send + Sync + SampleUniform {
    fn interval(&self, percentage: f64) -> (Self, Self);
    fn round(&self, percentile_gap: usize) -> Self;
}

impl DataTrait for usize {
    fn interval(&self, percentage: f64) -> (usize, usize) {
        let difference = ((*self as f64) * (percentage / 100.0)) as usize;
        (self - difference, self + difference)
    }
    fn round(&self, percentile_gap: usize) -> usize {
        (self / percentile_gap) * percentile_gap
    }
}
impl DataTrait for f64 {
    fn interval(&self, percentage: f64) -> (f64, f64) {
        let difference = self * (percentage / 100.0);
        (self - difference, self + difference)
    }
    fn round(&self, _percentile_gap: usize) -> f64 {
        *self
    }
}
