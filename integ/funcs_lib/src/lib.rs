pub mod rect;
pub mod trap;
pub mod simp;

pub use self::rect::left::left_c;
pub use self::rect::center::center_c;
pub use self::rect::right::right_c;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}

pub struct CalcErr(String);
pub struct Req {
    pub l: f32,
    pub r: f32,
    pub n: u32,
    pub eps: f32,
    pub f: fn(f32) -> f32,
}

