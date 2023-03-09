use std::f64::consts::E;

use crate::calc_util::Data;

pub enum Funcs {
    Log,
    Poly,
    Sinh,
    PolySin,
}

pub fn get_func_name(func: Funcs) -> String {
    match func {
        Funcs::Log => String::from("ln(x) - x + x^2"),
        Funcs::Poly => String::from("-7x^5 + x^3 -1"),
        Funcs::Sinh => String::from("5sinh(x) - 1"),
        Funcs::PolySin => String::from("sin(2x) - cos(x)"),
    }
}

pub fn get_func_by_type(func: &Funcs) -> for<'a> fn(&'a f64) -> f64 {
    match func {
        Funcs::Log => |x: &f64| -> f64 { log_func(x) },
        Funcs::Poly => |x: &f64| -> f64 { poly_func(x) },
        Funcs::Sinh => |x: &f64| -> f64 { sinh_func(x) },
        Funcs::PolySin => |x: &f64| -> f64 { polysin_func(x) },
    }
}

pub fn log_func(x: &f64) -> f64 {
    x.log(E) - x + x * x
}

pub fn poly_func(x: &f64) -> f64 {
    -7.0 * x.powi(5) + x.powi(3) - 1.0
}

pub fn sinh_func(x: &f64) -> f64 {
    5.0 * (x).sinh() - 1.0
}

pub fn polysin_func(x: &f64) -> f64 {
    (2.0 * x).sin() - x.cos()
}

pub fn calculate_root_number(data: &Data) -> u32 {
    let mut count: u32 = 0;
    for i in (data.l as i64) * 1000..(data.r as i64) * 1000 {
        if (data.func)(&((i as f64) / 1000.0)) * (data.func)(&(((i as f64) + 1.0) / 1000.0)) < 0.0 {
            count += 1;
        }
    }
    count
}
