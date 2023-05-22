use crate::matrix::{self, MT};

pub struct Gauss {
    mt: matrix::MT<f32>,
    x_arr: Vec<f32>,
}

pub fn fact(n: usize) -> f32 {
    let mut ans = 1.0;
    for i in 1..n + 1 {
        ans *= i as f32;
    }
    ans
}

pub fn coef_first(i: usize, t: f32) -> f32 {
    let mut coef = t;
    if i == 0 {
        return 1.0;
    }
    if i == 1 {
        return t;
    }
    for j in 2..i {
        coef *= t + (-1.0_f32).powi((j + 1) as i32) * ((j / 2) as f32);
    }
    coef / fact(i)
}

pub fn coef_second(i: usize, t: f32) -> f32 {
    let mut coef = t;
    if i == 0 {
        return 1.0;
    }
    if i == 1 {
        return t;
    }
    for j in 2..i {
        coef *= t + (-1.0_f32).powi(j as i32) * ((j / 2) as f32);
    }
    coef / fact(i)
}

impl Gauss {
    pub fn new(mt: MT<f32>, x_arr: Vec<f32>) -> Self {
        Gauss {
            mt: mt,
            x_arr: x_arr,
        }
    }
    
    pub fn eval_stir(&self, x: f32) -> f32 {
        (self.eval_first(x) + self.eval_second(x)) / 2.0
    }

    pub fn eval_first(&self, x: f32) -> f32 {
        let inf = self.x_arr.len() + 1;
        let mut id: usize = inf;
        let mut ans = 0.0;
        for i in 0..self.x_arr.len() - 1 {
            if self.x_arr[i + 1] > x && self.x_arr[i] <= x {
                id = i;
                break;
            }
        }
        if id == inf {
            return 0.0;
        }

        let t = (x - self.x_arr[id]) / (self.x_arr[1] - self.x_arr[0]);

        for i in 0..id + 1 {
            ans += self.mt[id - (i + 1) / 2][i] * coef_first(i, t);
            if i + 1 < self.x_arr.len() {
                ans += self.mt[id - (i + 1) / 2][i + 1] * coef_first(i + 1, t);
            } else {
                return ans;
            }
        }
        ans
    }

    pub fn eval_second(&self, x: f32) -> f32 {
        let inf = self.x_arr.len() + 1;
        let mut id: usize = inf;
        let mut ans = 0.0;
        for i in 0..self.x_arr.len() - 1 {
            if self.x_arr[i + 1] > x && self.x_arr[i] <= x {
                id = i;
                break;
            }
        }
        if id == inf {
            return 0.0;
        }

        let t = (x - self.x_arr[id]) / (self.x_arr[1] - self.x_arr[0]);

        for i in 0..id + 1 {
            ans += self.mt[id - i / 2][i] * coef_second(i, t);
            if i + 1 < self.x_arr.len() {
                ans += self.mt[id - i / 2][i + 1] * coef_second(i + 1, t);
            } else {
                return ans;
            }
        }
        ans
    }
}
