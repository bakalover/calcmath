pub mod lagr;
pub mod newton;
pub mod validation;

mod matrix;
use matrix::MT;

pub fn finite_mt(nodes: &[f32], f: impl Fn(&f32) -> f32) -> MT<f32> {
    let nodes_applied: Vec<f32> = nodes.iter().map(|x| f(x)).collect();
    let mut m = MT::new(nodes.len(), nodes.len());

    for i in 0..nodes_applied.len() {
        m[i][0] = nodes_applied[i];
    }

    for col in 1..m.get_data().len() {
        for row in 0..m.get_data().len() - col {
            m[row][col] = m[row + 1][col - 1] - m[row][col - 1];
        }
    }

    m
}

#[cfg(test)]
mod tests {
    use std::io::{stdout, Write};

    use crate::{lagr::Lagr, newton::Newton};

    use super::*;

    #[test]
    fn it_works() {
        let m = finite_mt(&vec![1.0, 2.0, 3.0, 7.0, -3.0], |x| x * 2.0);
        for i in 0..m.get_data().len() {
            for j in 0..m.get_data()[0].len() {
                print!("{} ", m[i][j]);
            }

            stdout().flush().unwrap();
            println!(" ");
        }
    }
    #[test]
    fn lagr() {
        let x_arr = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y_arr: Vec<f32> = x_arr.iter().map(|x| x * 2.0).collect();
        let lagr = Lagr::new(x_arr, y_arr);
        println!("{}", lagr.eval(2.7));
    }

    #[test]
    fn newton() {
        let x_arr = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mt = finite_mt(&x_arr, |x| x * 2.0);
        let newton = Newton::new(mt, x_arr);
        println!("{}", newton.eval(3.66));
    }
}
