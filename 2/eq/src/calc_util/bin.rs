use super::{CustomError, Data, Outcome};
use crate::func_util::*;

pub fn calculate_bin(data: &Data) -> Result<Outcome, CustomError> {
    let mut iters = 0;
    let mut l = data.l;
    let mut r = data.r;
    let mut res = (l + r) / 2.0;
    let func = get_func_by_type(&data.func_type);
    res = (l + r) / 2.0;
    while func(&res).abs() > std::f64::EPSILON && (r - l) > data.eps
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
