use ndarray::{Array, Array1};
use ndarray_linalg::types::c64;
use std::ops::*;

#[derive(Debug, Clone)]
pub struct Plaintxt(pub Array1<c64>);

impl Plaintxt {
    pub fn new(vec: Array1<c64>) -> Plaintxt {
        Plaintxt(vec)
    }

    pub fn eval(&self, root: c64) -> c64 {
        let mut sum = c64::new(0f64, 0f64);
        for i in 0..self.0.len() {
            sum = sum + root.powu(i as u32) * self.0[i];
        }
        sum
    }

    #[allow(dead_code)]
    /// size function calculate below
    ///|h| = (a_0^2 + a_1^2 + ... + a_n^2 )^1/2
    pub fn size(&self) -> c64 {
        self.0
            .iter()
            .fold(c64::new(0f64, 0f64), |sum, a| sum + a.powi(2))
            .powf(0.5)
    }
}

impl Add for Plaintxt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Plaintxt(self.0 + rhs.0)
    }
}

impl Mul for Plaintxt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let d = self.0.len() + rhs.0.len() - 1;
        let mut poly = Array::zeros(d);

        for k in 0..=d {
            for i in 0..=k {
                if self.0.len() <= i || rhs.0.len() <= k - i {
                    continue;
                }
                poly[k] = poly[k] + self.0[i] * rhs.0[k - i];
            }
        }

        Plaintxt(poly)
    }
}

impl Div<usize> for Plaintxt {
    type Output = Self;
    fn div(self, rhs: usize) -> Self {
        Plaintxt::new(self.0.mapv(|x| x / rhs as f64))
    }
}