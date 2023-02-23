use crate::util::*;

use super::Data;

pub fn calculate_bin(data: &Data) -> f64{
    let mut a = data.a;
    let mut b = data.b;
    let mut res = (a+b)/2.0;
    let func = 
    match data.func{
        Funcs::Log => |x: &f64| -> f64{log_func(x)},
        Funcs::Poly => |x: &f64| -> f64{poly_func(x)},
        Funcs::Sin => |x: &f64| -> f64{sin_func(x)},
    };
    res = (a+b)/2.0;
    while func(&res).abs()>std::f64::EPSILON {
        if func(&res) * func(&b) < 0.0 {
            a = res;
        }
        else {
            b = res;
        }
        res = (a+b)/2.0;
    };
    res
}