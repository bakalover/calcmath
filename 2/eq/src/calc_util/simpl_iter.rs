use std::f64::{INFINITY, NEG_INFINITY};

use crate::func_util::{get_der1_by_type, get_func_by_type};

use super::{CustomError, Data, Outcome};

pub fn calculate_simpl_iter(data: &Data) -> Result<Outcome, CustomError> {
    let lambda: f64;
    let mut x_i;
    let mut x_ip1;
    let mut iters = 0;
    let f = get_func_by_type(&data.func_type);
    match get_lamda_part(data) {
        Ok(lp) => lambda = -(1.0 / lp),
        Err(_) => {
            return Err(CustomError(format!(
                "Невозможно проверить достаточное условие сходимости!"
            )))
        }
    }
    x_i = data.l;
    x_ip1 = phix(f, lambda, x_i, true);
    if (x_i < x_ip1) {
        while (x_i - x_ip1).abs() > data.eps {
            iters += 1;
            x_i = x_ip1;
            x_ip1 = phix(f, lambda, x_i, true);
        }
    } else {
        while (x_i - x_ip1).abs() > data.eps {
            iters += 1;
            x_i = x_ip1;
            x_ip1 = phix(f, lambda, x_i, false);
        }
    }
    Ok(Outcome {
        ans: x_ip1,
        iters: iters,
        f: f(&x_ip1),
    })
}

fn get_lamda_part(data: &Data) -> Result<f64, ()> {
    let mut max_val: f64 = 0.0;
    let func_der1 = get_der1_by_type(&data.func_type);
    for i in ((data.l * 1000.0) as i64)..((data.r * 1000.0) as i64) {
        if func_der1(&((i as f64) / 1000.0)).abs() > max_val {
            max_val = func_der1(&((i as f64) / 1000.0)).abs();
        }
    }
    match max_val {
        INFINITY => return Err(()),
        NEG_INFINITY => return Err(()),
        _ => return Ok(max_val),
    }
}

fn phix(f: fn(&f64) -> f64, lambda: f64, arg: f64, sign: bool) -> f64 {
    if sign {
        return arg + lambda * f(&arg);
    } else {
        return arg - lambda * f(&arg);
    }
}
