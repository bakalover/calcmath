pub mod adams;
pub mod runge;
pub mod super_euler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{}",
            super_euler::eval((1.0, 1.5), -1.0, 0.1, |x, y| y + (1.0 + x) * y * y, 0.00001)
        );
    }
}
