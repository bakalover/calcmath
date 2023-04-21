use crate::poly;

use super::CalcError;

pub fn log(arr: &[(f32, f32)]) -> Result<((f32, f32), f32), CalcError> {
    let mut m_x: f32 = f32::MAX;

    for pt in arr {
        if pt.0 < m_x {
            m_x = pt.0;
        }
    }

    let arr_ln: Vec<(f32, f32)> = arr
        .into_iter()
        .map(|pt| ((pt.0 - m_x + 1.0).ln(), pt.1))
        .collect();

    let linear = poly(arr_ln.as_slice(), 2);
    match linear {
        Ok(val) => {
            return Ok(((val[0], val[1]), m_x - 1.0));
        }
        Err(err) => return Err(err),
    }
}
