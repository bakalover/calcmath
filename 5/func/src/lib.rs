pub mod lagr;
pub mod matrix;
pub mod newton;
pub mod validation;

pub use matrix::MT;
pub use validation::*;

pub fn finite_mt(x_args: &[f32], y_args: &[f32]) -> MT<f32> {
    let mut m = MT::new(x_args.len(), x_args.len());

    for i in 0..y_args.len() {
        m[i][0] = y_args[i];
    }

    for col in 1..m.get_data().len() {
        for row in 0..m.get_data().len() - col {
            m[row][col] = m[row + 1][col - 1] - m[row][col - 1];
        }
    }

    m
}