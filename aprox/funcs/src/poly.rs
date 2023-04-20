use std::{io::LineWriter, ops::Mul};

use nalgebra::{Matrix3x1, Matrix2, Matrix2x1, MatrixMN, SMatrix, Const, Dim};

use super::{CalcError,Type};

pub fn poly(arr : &[(f32,f32)], tp: Type) -> Result<Vec<f32>, CalcError>{
    match tp {
        Type::Linear => return lin(arr),
        Type::Quadratic => quad(arr),
        Type::Cube => cube(arr), 
    }
}

fn lin(arr : &[(f32,f32)]) -> Result<Vec<f32>, CalcError>{
    let A: SMatrix<f32,2 ,2> = SMatrix::from_fn(|i,j| -> f32{arr.into_iter().map(|pt| -> f32{pt.0.powi((i+j) as i32)}).sum()});
    let A_inv = match A.try_inverse(){
        Some(val) => val,
        None => return Err(CalcError("Linear: det = 0".to_string())),
    };

    let y: SMatrix<f32,2,1> = SMatrix::from_fn(|i,j| -> f32{arr.into_iter().map(|pt| -> f32{pt.0.powi(i as i32) * pt.1}).sum()});
    return Ok(A_inv.mul(y).as_mut_slice().to_vec());
}

fn quad(arr : &[(f32,f32)]) -> Result<Vec<f32>, CalcError>{
    let A: SMatrix<f32,3 ,3> = SMatrix::from_fn(|i,j| -> f32{arr.into_iter().map(|pt| -> f32{pt.0.powi((i+j) as i32)}).sum()});
    let A_inv = match A.try_inverse(){
        Some(val) => val,
        None => return Err(CalcError("Linear: det = 0".to_string())),
    };
    let y: SMatrix<f32,3,1> = SMatrix::from_fn(|i,j| -> f32{arr.into_iter().map(|pt| -> f32{pt.0.powi(i as i32) * pt.1}).sum()});
    return Ok(A_inv.mul(y).as_mut_slice().to_vec());
}

fn cube(arr : &[(f32,f32)]) -> Result<Vec<f32>, CalcError>{
    let A: SMatrix<f32,4 ,4> = SMatrix::from_fn(|i,j| -> f32{arr.into_iter().map(|pt| -> f32{pt.0.powi((i+j) as i32)}).sum()});
    let A_inv = match A.try_inverse(){
        Some(val) => val,
        None => return Err(CalcError("Linear: det = 0".to_string())),
    };
    let y: SMatrix<f32,4,1> = SMatrix::from_fn(|i,j| -> f32{arr.into_iter().map(|pt| -> f32{pt.0.powi(i as i32) * pt.1}).sum()});
    return Ok(A_inv.mul(y).as_mut_slice().to_vec());
}





// use super::CalcError;
// pub fn lin(arr: &[(f32,f32)]) -> Result<(f32,f32),CalcError>{
//     let (a, b):( f32,f32);
//     let mut sx: f32 = 0.0;
//     let mut sy: f32 = 0.0;
//     let mut sxx: f32 = 0.0;
//     let mut sxy: f32 = 0.0;


//     for point in arr{
//         sx += point.0;
//         sy += point.1;
//         sxx += point.0.powi(2);
//         sxy += point.0 * point.1;
//     }

//     if(sxx * (arr.len() as f32) - sx * sx <= f32::EPSILON){
//         return Err(CalcError("Знаменатель коэффициентов линейной аппроксимации равен нулю".to_string()));
//     }

//     a = (sxy * (arr.len() as f32) - sx * sy)/(sxx * (arr.len() as f32) - sx * sx);
//     b = (sxx * sy - sx * sxy)/(sxx * (arr.len() as f32) - sx * sx);

//     Ok((a,b))
// }