use std::ops::Mul;

use nalgebra::DMatrix;

use super::CalcError;

pub fn poly(arr: &[(f32, f32)], dim: usize) -> Result<Vec<f32>, CalcError> {
    let A: DMatrix<f32> = DMatrix::from_fn(dim, dim, |i, j| -> f32 {
        arr.into_iter()
            .map(|pt| -> f32 { pt.0.powi((i + j) as i32) })
            .sum()
    });
    let A_inv = match A.try_inverse() {
        Some(val) => val,
        None => return Err(CalcError("ERR: Det = 0".to_string())),
    };

    let y: DMatrix<f32> = DMatrix::from_fn(dim, 1, |i, j| -> f32 {
        arr.into_iter()
            .map(|pt| -> f32 { pt.0.powi(i as i32) * pt.1 })
            .sum()
    });
    return Ok(A_inv.mul(y).as_mut_slice().to_vec());
}
