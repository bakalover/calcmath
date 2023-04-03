use crate::{CalcErr, Req};
pub fn center_c(req: &Req) -> Result<f32, CalcErr> {
    let (mut ans, dx) = (0.0, (req.r - req.l) / (req.n as f32));

    for i in ((req.l * (req.n as f32)) as u32)..((req.r * (req.n as f32)) as u32) {
        ans += (req.f)(((i as f32) + dx / 2.0) / (req.n as f32)) * dx;
    }

    Ok(ans)
}
