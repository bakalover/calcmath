use crate::matrix::{self, MT};

pub struct Newton {
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

impl Newton {
    pub fn new(mt: MT<f32>, x_arr: Vec<f32>) -> Self {
        Newton {
            mt: mt,
            x_arr: x_arr,
        }
    }

    pub fn eval(&self, x: f32) -> f32 {
        // Проверка На принадлежность нужной половине отрезка
        if self.x_arr[self.x_arr.len() / 2] >= x {
            return self.left(x);
        } else {
            return self.right(x);
        }
    }

    fn left(&self, x: f32) -> f32 {
        let mut t: f32 = 0.0;
        let mut id: usize = 0;
        let h = self.x_arr[1] - self.x_arr[0];
        let mut ans: f32 = 0.0;

        for i in 0..self.x_arr.len() - 1 {
            if self.x_arr[i] < x && self.x_arr[i + 1] > x {
                t = (x - self.x_arr[i]) / h;
                id = i;
                break;
            }
        }

        for i in 0..self.x_arr.len() {
            let mut prod = self.mt[id][i];
            for j in 0..i {
                prod *= t - j as f32;
            }
            ans += prod / fact(i);
        }
        ans
    }

    fn right(&self, x: f32) -> f32 {
        let h = self.x_arr[1] - self.x_arr[0];
        let t = (x - self.x_arr[self.x_arr.len() - 1]) / h;
        let mut ans: f32 = 0.0;

        for i in 0..self.x_arr.len() {
            let mut prod = self.mt[self.x_arr.len() - 1 - i][i]; // Лесенка
            for j in 0..i {
                prod *= t + j as f32;
            }
            ans += prod / fact(i);
        }
        ans
    }
}
