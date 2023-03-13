use super::{CalcError, Data, Outcome};
use crate::func_util::*;

pub fn calculate_newton(data: &Data) -> Result<Outcome, CalcError> {
    let mut x_i;
    let mut x_i_1 = match get_start(&data) {
        Ok(numb) => numb,
        Err(_) => {
            return Err(CalcError(format!(
                "Невозможно проверить достаточное условие сходимости!"
            )))
        }
    };
    let mut iters: u64 = 0;
    let func = get_func_by_type(&data.func_type);
    let func_der1 = get_der1_by_type(&data.func_type);

    while func(&x_i_1).abs() > data.eps {
        iters += 1;
        x_i = x_i_1 - func(&x_i_1) / func_der1(&x_i_1);
        x_i_1 = x_i;
    }

    Ok(Outcome {
        ans: x_i_1,
        iters: iters,
        f: func(&x_i_1),
    })
}

fn get_start(data: &Data) -> Result<f64, ()> {
    let func = get_func_by_type(&data.func_type);
    let func_der2 = get_der2_by_type(&data.func_type);
    for i in ((data.l * 1000.0) as i64)..((data.r * 1000.0) as i64) {
        if (func(&((i as f64) / 1000.0)) * func_der2(&((i as f64) / 1000.0)) > 0.0) {
            return Ok((i as f64) / 1000.0);
        }
    }
    Err(())
}
