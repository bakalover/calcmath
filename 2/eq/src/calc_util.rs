use crate::func_util::{Funcs, calculate_root_number};

use self::{bin::calculate_bin, newton::calculate_newton, simpl_iter::calculate_simpl_iter};

pub mod bin;
pub mod newton;
pub mod newton_multi;
pub mod simpl_iter;
pub enum Methods {
    Bin,
    Newton,
    NewtonMulti,
    Simpl,
}

pub struct CalcError(pub String);

pub struct Outcome {
    pub ans: f64,
    pub iters: u64,
    pub f: f64,
}
pub struct Data {
    pub method: Methods,
    pub func_type: Funcs,
    pub l: f64,
    pub r: f64,
    pub eps: f64,
}

pub fn calculate(data: &Data) -> Result<Outcome, CalcError> {
    let roots = calculate_root_number(data);

    if roots > 1 || roots == 0 {
        return Err(CalcError(format!(
            "Найденное число корней: {} не соответствует 1!",
            roots
        )));
    }
    match data.method {
        Methods::Bin => calculate_bin(&data),
        Methods::Newton => calculate_newton(&data),
        Methods::Simpl => calculate_simpl_iter(&data),
        Methods::NewtonMulti => todo!(),
    }
}
