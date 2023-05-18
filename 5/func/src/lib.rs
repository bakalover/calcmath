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

// #[cfg(test)]
// mod tests {
//     use std::io::{stdout, Write};

//     use draw::draw;

//     use crate::{lagr::Lagr, newton::Newton};

//     use super::*;

//     #[test]
//     fn it_works() {
//         let m = finite_mt(&vec![1.0, 2.0, 3.0, 7.0, -3.0], |x| x * 2.0);
//         for i in 0..m.get_data().len() {
//             for j in 0..m.get_data()[0].len() {
//                 print!("{} ", m[i][j]);
//             }

//             stdout().flush().unwrap();
//             println!(" ");
//         }
//     }
//     #[test]
//     fn lagr() {
//         let x_arr = vec![1.0, 2.0, -9.0, 4.0, 51.0];
//         let y_arr: Vec<f32> = x_arr.iter().map(|x| x * x - x + x * x).collect();
//         let pts: Vec<(f32, f32)> = x_arr
//             .iter()
//             .zip(
//                 x_arr
//                     .iter()
//                     .map(|x| x * x - x + x * x)
//                     .collect::<Vec<f32>>()
//                     .iter(),
//             )
//             .map(|(&a, &b)| (a, b))
//             .collect();
//         let lagr = Lagr::new(x_arr, y_arr);
//         draw(
//             &pts,
//             vec![|x| lagr.eval(x)],
//             "/home/bakalover/code/calcmath/5/out/lagr.png".to_string(),
//         );
//         println!("{}", lagr.eval(2.7));
//     }

//     #[test]
//     fn newton() {
//         let x_arr = vec![1.0, 4.0, 7.0, 10.0, 13.0];
//         let mt = finite_mt(&x_arr, |x| x * x - x + x * x);
//         let pts: Vec<(f32, f32)> = x_arr
//             .iter()
//             .zip(
//                 x_arr
//                     .iter()
//                     .map(|x| x * x - x + x * x)
//                     .collect::<Vec<f32>>()
//                     .iter(),
//             )
//             .map(|(&a, &b)| (a, b))
//             .collect();
//         let newton = Newton::new(mt, x_arr);
//         println!("{}", newton.eval(3.4543));
//         println!("{}", newton.eval(3.0));
//         draw(
//             &pts,
//             vec![|x| newton.eval(x)],
//             "/home/bakalover/code/calcmath/5/out/new.png".to_string(),
//         );
//     }
// }
