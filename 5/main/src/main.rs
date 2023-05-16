use std::io::{stdout, Write};

use func::finite_mt;

fn main() {
    println!("Hello, world!");
    let m = finite_mt(&vec![1.0, 2.0, 3.0], |x| x * 2.0);
    for i in 0..m.get_data().len() {
        for j in 0..m.get_data()[0].len() {
            print!("{} ", m[i][j]);
        }
        stdout().flush().unwrap();
        println!(" ");
    }
}
