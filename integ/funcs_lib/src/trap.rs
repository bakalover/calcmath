use crate::Req;
pub fn trap_c(req: &Req) -> f64 {
    let dx = (req.r - req.l) / (req.n as f64);
    let mut ans = 0.0;

    let mut x_i = req.l;

    while x_i < req.r {
        ans += (req.f)(x_i);
        x_i += dx;
    }
    
    ans *= 2.0;
    ans -= (req.f)(req.l);
    ans -= (req.f)(req.r);
    ans*= dx/2.0;
    ans
}
