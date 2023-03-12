use std::f64::consts::E;

use crate::calc_util::Data;

pub enum Funcs {
    Log,
    Poly,
    Sinh,
    PolySin,
}

pub fn get_func_name(func: &Funcs) -> String {
    match func {
        Funcs::Log => String::from("ln(x) - x + x^2"),
        Funcs::Poly => String::from("-7x^5 + x^3 -1"),
        Funcs::Sinh => String::from("5sinh(x) - 1"),
        Funcs::PolySin => String::from("sin(2x) - cos(x)"),
    }
}

pub fn get_func_by_type(func: &Funcs) -> fn(&f64) -> f64 {
    match func {
        Funcs::Log => |x: &f64| -> f64 { log_func(x) },
        Funcs::Poly => |x: &f64| -> f64 { poly_func(x) },
        Funcs::Sinh => |x: &f64| -> f64 { sinh_func(x) },
        Funcs::PolySin => |x: &f64| -> f64 { polysin_func(x) },
    }
}

pub fn get_der1_by_type(func: &Funcs) -> fn(&f64) -> f64 {
    match func {
        Funcs::Log => |x: &f64| -> f64 { log_der1_func(x) },
        Funcs::Poly => |x: &f64| -> f64 { poly_der1_func(x) },
        Funcs::Sinh => |x: &f64| -> f64 { sinh_der1_func(x) },
        Funcs::PolySin => |x: &f64| -> f64 { polysin_der1_func(x) },
    }
}
//for<'a> fn(&'a f64) -> f64
pub fn get_der2_by_type(func: &Funcs) -> fn(&f64) -> f64 {
    match func {
        Funcs::Log => |x: &f64| -> f64 { log_der2_func(x) },
        Funcs::Poly => |x: &f64| -> f64 { poly_der2_func(x) },
        Funcs::Sinh => |x: &f64| -> f64 { sinh_der2_func(x) },
        Funcs::PolySin => |x: &f64| -> f64 { polysin_der2_func(x) },
    }
}

pub fn log_func(x: &f64) -> f64 {
    x.log(E) - x + x * x
}

pub fn log_der1_func(x: &f64) -> f64 {
    1.0 / x - 1.0 + 2.0 * x
}

pub fn log_der2_func(x: &f64) -> f64 {
    -1.0 / (x * x) + 2.0
}

pub fn poly_func(x: &f64) -> f64 {
    -7.0 * x.powi(5) + x.powi(3) - 1.0
}

pub fn poly_der1_func(x: &f64) -> f64 {
    -35.0 * x.powi(4) + 3.0 * x.powi(2)
}

pub fn poly_der2_func(x: &f64) -> f64 {
    -140.0 * x.powi(3) + 6.0 * x
}

pub fn sinh_func(x: &f64) -> f64 {
    5.0 * (x).sinh() - 1.0
}

pub fn sinh_der1_func(x: &f64) -> f64 {
    5.0 * (x).cosh()
}

pub fn sinh_der2_func(x: &f64) -> f64 {
    5.0 * (x).sinh()
}

pub fn polysin_func(x: &f64) -> f64 {
    (2.0 * x).sin() - x.cos()
}

pub fn polysin_der1_func(x: &f64) -> f64 {
    2.0 * (2.0 * x).cos() + x.sin()
}

pub fn polysin_der2_func(x: &f64) -> f64 {
    -4.0 * (2.0 * x).sin() + x.cos()
}

pub fn calculate_root_number(data: &Data) -> u32 {
    let mut count: u32 = 0;
    let mut acc_zeros: u32 = 0;
    let func = get_func_by_type(&data.func_type);
    for i in ((data.l * 1000.0) as i64)..((data.r * 1000.0) as i64) {
        if func(&((i as f64) / 1000.0)) * func(&(((i as f64) + 1.0) / 1000.0)) < 0.0 {
            count += 1;
        }
        if func(&((i as f64) / 1000.0)) * func(&(((i as f64) + 1.0) / 1000.0)) == 0.0 {
            acc_zeros += 1;
        }
    }
    count + acc_zeros / 2
}
