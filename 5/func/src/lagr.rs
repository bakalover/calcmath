pub struct Lagr {
    x_arr: Vec<f32>,
    y_arr: Vec<f32>,
}

impl Lagr {
    pub fn new(x_arr: Vec<f32>, y_arr: Vec<f32>) -> Self {

        Lagr {
            x_arr: x_arr,
            y_arr: y_arr,
        }
    }
    pub fn eval(&self, x: f32) -> f32 {
        let mut ans = 0.0;
        for i in 0..self.y_arr.len() {
            let mut prod = 1.0;
            for j in 0..self.x_arr.len() {
                if i != j {
                    assert_ne!(self.x_arr[i], self.x_arr[j], "Same x arguments -> division by zero in Lagrange formula");
                    prod *= (x - self.x_arr[j]) / (self.x_arr[i] - self.x_arr[j]);
                }
            }
            ans += self.y_arr[i] * prod;
        }
        ans
    }
}
