use std::f64::consts::E;

pub enum Funcs {
    Log,
    Poly,
    Sin,
}

pub fn get_func_name(func: Funcs) -> String {
    match func {
        Funcs::Log => String::from("ln(x) - x + x^2"),
        Funcs::Poly => String::from("-7x^5 + x^3 -1"),
        Funcs::Sin => String::from("sin(x/3) - cos(x)"),
    }
}

pub fn log_func(x:&f64) -> f64{
    x.log(E) - x + x*x
}

pub fn poly_func(x: &f64) -> f64{
    -7.0*x.powi(5) + x.powi(3) - 1.0
}

pub fn sin_func(x: &f64) -> f64{
    (x/3.0).sin() - x.cos()
}