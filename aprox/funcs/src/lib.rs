pub mod poly;

pub use poly::poly;


pub struct CalcError(String);
pub enum Type {
    Linear,
    Quadratic,
    Cube,
}
