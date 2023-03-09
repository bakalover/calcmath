use crate::func_util::{get_func_by_type, Funcs};

use self::{bin::calculate_bin, newton::calculate_newton};

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

pub struct CustomError(pub String);

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
    pub eps: u32,
}

pub fn calculate(data: &Data) -> Result<Outcome, CustomError> {
    match data.method {
        Methods::Bin => calculate_bin(&data),
        Methods::Newton => calculate_newton(&data),
        Methods::NewtonMulti => todo!(),
        Methods::Simpl => todo!(),
    }
}
