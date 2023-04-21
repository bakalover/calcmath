pub mod poly;
pub mod step;
pub mod exp;
pub mod log;

pub use poly::poly;
pub use step::step;
pub use exp::exp;
pub use log::log;

pub struct CalcError(String);