use std::io::stdin;

use funcs::{exp, log, poly, step};

fn main() {
    // let a: Vec<(f32, f32)> = vec![
    //     (1.0, 1.0),
    //     (1.5, 1.5),
    //     (5.0, 7.0),
    //     (8.0, 20.0),
    //     (-10.0, -3.0),
    //     (20.0, 30.0),
    //     (100.0, 200.0),
    // ];
    println!("Введите число точек N (8 <= N <= 12): ");

    let N: usize;
    let N_res: Result<usize, _> = stdin()
        .lines()
        .next()
        .expect("End of input has been detected!")
        .expect("Problems while reading string")
        .parse();

    match N_res {
        Err(_) => {
            println!("Число точек - не число!");
            return;
        }
        Ok(res) => N = res,
    }

    if N > 12 || N < 8 {
        println!("Число точек вне диапазона!");
        return;
    }

    let mut pts: Vec<(f32, f32)> = Vec::new();

    println!("\nВведите точки в формате: x y");

    for i in 0..N {
        let pt: Result<Vec<f32>, _> = stdin()
            .lines()
            .next()
            .expect("End of input has been detected!")
            .expect("Problems while reading string")
            .split(" ")
            .map(|s| -> Result<f32, _> { s.to_string().parse() })
            .collect();

        match pt {
            Err(_) => {
                println!("Одна или две координаты - не числа!");
                return;
            }
            Ok(res) => {
                if res.len() != 2 {
                    println!("У нас двухмерное измерение!");
                    return;
                }
                pts.push((res[0], res[1]))
            }
        }
    }

    let linear = match poly(pts.as_slice(), 4) {
        Ok(val) => val,
        Err(err) => return,
    };
    for i in linear {
        println!("{}", i)
    }
    // println!("{} {} {} ", linear.0 .0, linear.0 .1, linear.1);
}
// 1 1
// 1.5 1.5
// 5 7
// 8 20
// -10 -3
// 20 30
// 100 2
// 5 5
// 6 6
//
