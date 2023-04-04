use crate::Req;
pub fn center_c(req: &Req) -> f64 {
    let (mut ans, dx) = (0.0, (req.r - req.l) / (req.n as f64));
    let mut x_i = req.l + dx/2.0;

    while x_i < req.r {
        ans += (req.f)(x_i) * dx;
        x_i += dx;
    }

    ans
}
