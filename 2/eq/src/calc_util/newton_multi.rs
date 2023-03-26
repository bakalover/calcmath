use crate::draw::draw_multi;

pub fn calculate_newton_multi(x: f64, y: f64, eps: u32, sys_type: i32) {
    let mut iter = 0;
    let mut x: f64 = x;
    let mut y: f64 = y;
    let epsf = 1.0 / (eps as f64);
    let mut dx: f64 = 1000000.0;
    let mut dy: f64 = 1000000.0;
    match sys_type {
        1 => {
            while dx.abs() > epsf || dy.abs() > epsf || ((x - y).sin()).abs() > epsf  || ((x + y).cos()).abs() > epsf{
                iter += 1;
                dy = ((x - y).sin() * (x + y).sin() + (x + y).cos() * (x - y).cos())
                    / ((x - y).cos() + (x - y).cos() * (x + y).sin());
                dx = (dy * (x - y).cos() - (x - y).sin()) / ((x - y).cos());
                x += dx;
                y += dy;
            }
        }
        2 => {
            while dx.abs() > epsf
                || dy.abs() > epsf
                || (x * x + y * y - 30.0).abs() > epsf
                || ((x / 2.0).sin() - y).abs() > epsf
            {
                iter += 1;
                dx = (30.0 + 2.0 * y * y - x * x - y * y - 2.0 * y * ((x / 2.0).sin()))
                    / (2.0 * x + y * ((x / 2.0).cos()));
                dy = (1.0 / 2.0) * ((x / 2.0).cos()) * dx - y + (x / 2.0).sin();
                x += dx;
                y += dy;
            }
        }
        _ => println!("problems"),
    }
    println!("Число итераций: {}\n", iter);
    println!("\nПолученный вектор: \nx: {} \ny: {}\n", x,y);
    println!("\nПолученный вектор погрешностей: \nx: {} \ny: {}\n", dx.abs(),dy.abs());
    match draw_multi(x, y, sys_type) {
        Ok(_) => println!("Проверьте ваш график в папке out"),
        Err(_) => println!("Невозможно построить график!"),
    }
}
