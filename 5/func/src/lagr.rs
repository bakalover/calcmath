pub struct Lagr {
    X: Vec<f32>,
    Y: Vec<f32>,
}

impl Lagr {
    pub fn new(X: &[f32], Y: &[f32]) -> Self {
        assert_eq!(X.len(), Y.len());
        Lagr {
            X: X.clone().to_vec(),
            Y: Y.clone().to_vec(),
        }
    }
    pub fn eval(&self, x: f32) -> f32 {
        let mut ans = 0.0;
        for i in 0..self.Y.len() {
            let mut prod = 1.0;
            for j in 0..self.X.len() {
                if (i != j) {
                    prod *= (x - self.X[j]) / (self.X[i] - self.X[j]);
                }
            }
            ans += self.Y[i] * prod;
        }
        ans
    }
}
