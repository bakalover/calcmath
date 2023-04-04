use crate::Req;
pub fn trap_c(req: &Req) -> f64 {
    let dx = (req.r - req.l) / (req.n as f64);
    let mut ans = 0.0;

    for i in ((req.l * (req.n as f64)) as u32)..((req.r * (req.n as f64)) as u32) {
        ans += (req.f)((i as f64) / (req.n as f64));
    }

    ans *= 2.0;
    ans -= (req.f)(req.l);
    ans -= (req.f)(req.r);
    ans*= dx/2.0;
    ans
}
