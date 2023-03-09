use super::{CustomError, Data, Outcome};
use crate::func_util::*;

pub fn calculate_bin(data: &Data) -> Result<Outcome, CustomError> {
    let roots = calculate_root_number(data);

    if roots > 1 || roots == 0 {
        return Err(CustomError(format!(
            "Найденное число корней: {} не соответствует 1!",
            roots
        )));
    }

    let mut iters = 0;
    let mut l = data.l;
    let mut r = data.r;
    let mut res = (l + r) / 2.0;
    let func = get_func_by_type(&data.func_type);
    res = (l + r) / 2.0;
    while func(&res).abs() > std::f64::EPSILON && (r - l) > 1.0 / ((10 as u64).pow(data.eps) as f64)
    {
        iters += 1;
        if func(&res) * func(&r) < 0.0 {
            l = res;
        } else {
            r = res;
        }
        res = (l + r) / 2.0;
    }

    Ok(Outcome {
        ans: res,
        iters: iters,
        f: func(&res),
    })
}
