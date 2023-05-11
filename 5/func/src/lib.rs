pub mod lagr;
pub mod newton;

mod matrix;
use matrix::MT;

pub fn finiteMT(nodes: &[f32], f: impl Fn(&f32) -> f32) -> MT<f32> {
    let nodes_applied: Vec<f32> = nodes.iter().map(|x| f(x)).collect();
    let mut M = MT::new(nodes.len(), nodes.len());

    for i in 0..nodes_applied.len() {
        M[i][0] = nodes_applied[i];
    }

    for col in 1..M.get_data().len() {
        for row in 0..M.get_data().len() - col {
            M[row][col] = M[row + 1][col - 1] - M[row][col - 1];
        }
    }

    M
}

#[cfg(test)]
mod tests {
    use std::io::{stdout, Write};

    use crate::{
        lagr::Lagr,
        newton::{fact, Newton},
    };

    use super::*;

    #[test]
    fn it_works() {
        let M = finiteMT(&vec![1.0, 2.0, 3.0, 7.0, -3.0], |x| x * 2.0);
        for i in 0..M.get_data().len() {
            for j in 0..M.get_data()[0].len() {
                print!("{} ", M[i][j]);
            }

            stdout().flush();
            println!(" ");
        }
    }
    #[test]
    fn lagr() {
        let X = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let Y: Vec<f32> = X.iter().map(|x| x * 2.0).collect();
        let Lagr = Lagr::new(&X, &Y);
        println!("{}", Lagr.eval(2.7));
    }

    #[test]
    fn newton() {
        let X = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let MT = finiteMT(&X, |x| x * 2.0);
        let newton = Newton::new(MT, X);
        println!("{}", newton.eval(3.66));
    }
}
