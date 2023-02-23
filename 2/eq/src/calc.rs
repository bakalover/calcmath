use crate::util::Funcs;

use self::bin::calculate_bin;

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

pub struct Data{
    pub(crate) method: Methods,
    pub(crate) func: Funcs,
    pub(crate) a: f64,
    pub(crate) b: f64,
}

pub fn calculate(data: &Data) -> f64{
    let mut res: f64;
    match data.method {
        Methods::Bin => calculate_bin(&data),
        Methods::Newton => todo!(),
        Methods::NewtonMulti => todo!(),
        Methods::Simpl => todo!(), 
    }
}