use std::f64::consts::E;

pub enum Funcs {
    A,
    B,
    C,
    D,
}

pub fn get_func(opt: Funcs) -> fn(f64) -> f64 {
    match opt {
        Funcs::A => |x| -> f64 { 5.0 * E.powf(x) + x.sin() },
        Funcs::B => |x| -> f64 { x.powf(3.0) + 12.0 * x.powf(2.0) + 1.0 },
        Funcs::C => |x| -> f64 { x.atan() + 5.0 * x },
        Funcs::D => |x| -> f64 { (x.powf(2.0) + x.powf(4.0)).sqrt() },
    }
}
