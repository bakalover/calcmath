pub mod bin;
pub mod newton;
pub mod newton_multi;
pub mod simpl_iter;

enum Methods {
    Bin,
    Newton,
    NewtonMulti,
    Simpl,
}

pub struct Data{
    method: Methods,
    a_0: f64,
    b_0: f64,
}

pub fn calculate(data: &Data) -> f64{
    let mut res: f64;
    match data.method {
        Methods::Bin => todo!(),
        Methods::Newton => todo!(),
        Methods::NewtonMulti => todo!(),
        Methods::Simpl => todo!(), 
    }
    res
}