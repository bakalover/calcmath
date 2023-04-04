use std::ops::Rem;

use crate::Req;
pub fn simp_c(req: &Req) -> f64 {
    let dx = (req.r - req.l) / (req.n as f64);
    let (mut x_i, mut y_i, mut ans) = (req.l, 0.0, 0.0);
    let mut count:u64 = 0;

    while x_i < req.r {
        y_i = (req.f)(x_i);
        if x_i == req.l || x_i == req.r {
            ans += y_i;
        } else {
            match count.rem(2) {
                1 => ans += 4.0 * y_i,
                0 => ans += 2.0 * y_i,
                _ => print!("asd"),
            }
        }
        count += 1;

        x_i += dx;
    }

    ans *= dx / 3.0;

    ans
}
