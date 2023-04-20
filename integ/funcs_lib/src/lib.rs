pub mod funcs;
pub mod rect;
pub mod simp;
pub mod trap;

use rect::{center::center_c, left::left_c, right::right_c};
use simp::simp_c;
use trap::trap_c;

pub use self::funcs::*;

pub fn eval(req: &Req, opt: &Methods) -> f64 {
    match opt {
        Methods::Left => left_c(req),
        Methods::Right => right_c(req),
        Methods::Center => center_c(req),
        Methods::Trap => trap_c(req),
        Methods::Simp => simp_c(req),
    }
}

pub struct Req {
    pub l: f64,
    pub r: f64,
    pub n: u64,
    pub f: fn(f64) -> f64,
}

pub enum Methods {
    Left,
    Right,
    Center,
    Trap,
    Simp,
}
