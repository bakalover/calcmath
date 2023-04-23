use std::{
    f32::consts::E,
    io::{stdin, stdout, Write},
};

use funcs::{
    exp, log,
    metrics::{get_emp, print_metrics},
    poly, step,
};

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

    let linear = match poly(pts.as_slice(), 2) {
        Ok(val) => val,
        Err(err) => {
            println!("Невозможно определить линейную функцию: {}", err.0);
            Vec::new()
        }
    };
    println!("\nЛинейная функция: ({})x + ({})", linear[1], linear[0]);
    let linear_metrics = get_emp(&pts, |x| linear[1] * x + linear[0]);
    print_metrics(&linear_metrics);

    let quad = match poly(pts.as_slice(), 3) {
        Ok(val) => val,
        Err(err) => {
            println!("Невозможно определить квадратичную функцию: {}", err.0);
            Vec::new()
        }
    };
    println!(
        "\nКвадратичная функция: ({})x^2 + ({})x + ({})",
        quad[2], quad[1], quad[0]
    );
    let quad_metrics = get_emp(&pts, |x| quad[2] * x * x + quad[1] * x + quad[2]);
    print_metrics(&quad_metrics);

    let cube = match poly(pts.as_slice(), 4) {
        Ok(val) => val,
        Err(err) => {
            println!("Невозможно определить кубическую функцию: {}", err.0);
            Vec::new()
        }
    };
    println!(
        "\nКубическая функция: ({})x^3 + ({})x^2 + ({})x + ({})",
        cube[3], cube[2], cube[1], cube[0]
    );
    let cube_metrics = get_emp(&pts, |x| {
        cube[3] * x * x * x + cube[2] * x * x + cube[1] * x + cube[0]
    });
    print_metrics(&cube_metrics);

    let step = match step(pts.as_slice()) {
        Ok(val) => val,
        Err(err) => {
            println!("Невозможно определить Степенную функцию: {}", err.0);
            ((0.0, 0.0), (0.0, 0.0))
        }
    };
    println!(
        "\nСтепенная функция: ({})(x - ({}))^({}) - ({})",
        step.0 .0, step.1 .0, step.0 .1, step.1 .1
    );
    let step_metrics = get_emp(&pts, |x| {
        step.0 .0 * (x - step.1 .0).powf(step.0 .1) - step.1 .1
    });
    print_metrics(&step_metrics);

    let exp = match exp(pts.as_slice()) {
        Ok(val) => val,
        Err(err) => {
            println!("Невозможно определить экспоненциальную функцию: {}", err.0);
            ((0.0, 0.0), 0.0)
        }
    };
    println!(
        "\nЭкспоненциальная функция: ({})e^(({}) * x) - ({})",
        exp.0 .0, exp.0 .1, exp.1
    );
    let exp_metrics = get_emp(&pts, |x| exp.0 .0 * (E).powf(exp.0 .1 * x) - exp.1);
    print_metrics(&exp_metrics);

    let log = match log(pts.as_slice()) {
        Ok(val) => val,
        Err(err) => {
            println!("Невозможно определить Логарифмическую функцию: {}", err.0);
            ((0.0, 0.0), 0.0)
        }
    };
    println!(
        "\nЛогарифмическая функция: ({})ln(x - ({})) - ({})",
        log.0 .1, log.1, log.0 .0
    );
    let log_metrics = get_emp(&pts, |x| log.0 .1 * (x - log.1).ln() - log.0 .0);
    print_metrics(&log_metrics);

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
