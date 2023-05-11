use std::io::{stdout, Write};

use func::finite;

fn main() {
    println!("Hello, world!");
    let M = finite(&vec![1.0, 2.0, 3.0], |x| x * 2.0);
    for i in 0..M.get_data().len() {
        for j in 0..M.get_data()[0].len() {
            print!("{} ", M[i][j]);
        }
        stdout().flush();
        println!(" ");
    }
}
