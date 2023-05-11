use crate::matrix::{self, MT};

pub struct Newton {
    MT: matrix::MT<f32>,
    X: Vec<f32>,
}

pub fn fact(n: usize) -> f32 {
    let mut ans = 1.0;
    for i in 1..n + 1 {
        ans *= i as f32;
    }
    ans
}

impl Newton {
    pub fn new(MT: MT<f32>, X: Vec<f32>) -> Self {
        Newton { MT: MT, X: X }
    }
    pub fn eval(&self, x: f32) -> f32 {
        let mut t = (x - self.X[0]) / (self.X[1] - self.X[0]);
        let mut ans: f32 = 0.0;
        for i in 0..self.X.len() {
            let mut prod = self.MT[0][i];
            for j in 0..i {
                prod *= t - j as f32;
            }
            ans += prod / fact(i);
        }
        ans
    }
}
