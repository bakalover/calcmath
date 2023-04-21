use crate::poly;

use super::CalcError;

pub fn step(arr: &[(f32, f32)]) -> Result<((f32, f32), (f32, f32)), CalcError> {
    let (mut m_x, mut m_y): (f32, f32) = (f32::MAX, f32::MAX);

    for pt in arr {
        if pt.0 < m_x {
            m_x = pt.0;
        }
        if pt.1 < m_y {
            m_y = pt.1;
        }
    }

    let arr_ln: Vec<(f32, f32)> = arr
        .into_iter()
        .map(|pt| ((pt.0 - m_x + 1.0).ln(), (pt.1 - m_y + 1.0).ln()))
        .collect();

    let linear = poly(arr_ln.as_slice(), 2);
    match linear {
        Ok(val) => {
            return Ok(((val[0].exp(), val[1]), (m_x - 1.0, m_y - 1.0)));
        }
        Err(err) => return Err(err),
    }
}
