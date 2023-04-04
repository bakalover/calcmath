use std::ops::Rem;

use crate::Req;
pub fn simp_c(req: &Req) -> f64 {
    let (l, r) = (
        ((req.l * (req.n as f64)) as u32),
        ((req.r * (req.n as f64)) as u32),
    );
    let dx = (req.r - req.l) / (req.n as f64);
    let mut y_i;
    let mut ans = 0.0;

    for i in l..r {
        y_i = (req.f)((i as f64) / (req.n as f64));
        if i == 0 || i == r {
            ans += y_i;
        }
        match i.rem(2) {
            1 => ans += 4.0 * y_i,
            0 => ans += 2.0 * y_i,
            _ => print!("asd"),
        }
    }

    ans *= dx / 3.0;

    ans
}
