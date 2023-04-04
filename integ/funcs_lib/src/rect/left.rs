use crate::Req;
pub fn left_c(req: &Req) -> f64 {
    let (mut ans, dx) = (0.0, (req.r - req.l) / (req.n as f64));
    let mut x_i = req.l;

    while x_i < req.r {
        ans += (req.f)(x_i) * dx;
        x_i += dx;
    }

    ans
}
