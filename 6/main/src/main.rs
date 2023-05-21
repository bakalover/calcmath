use func::{runge, super_euler};

fn main() {
    println!(
        "{}",
        super_euler::eval(
            (1.0, 1.5),
            -1.0,
            0.1,
            |x, y| y + (1.0 + x) * y * y,
            0.0000001
        )
    );
    println!(
        "{}",
        runge::eval(
            (1.0, 1.5),
            -1.0,
            0.1,
            |x, y| y + (1.0 + x) * y * y,
            0.000000001
        )
    );
}
