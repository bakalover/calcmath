pub fn eval<F: Fn(f64, f64) -> f64>(
    bord: (f64, f64),
    y_0: f64,
    h: f64,
    f: F,
    eps: f64,
) -> Vec<(f32, f32)> {
    let mut full_ans = Vec::<(f32, f32)>::new();

    let mut ans_h: f64 = y_0;
    let mut ans_h_2: f64 = y_0;
    let mut x_i: f64 = bord.0;

    let (mut k1, mut k2, mut k3, mut k4): (f64, f64, f64, f64);

    full_ans.push((bord.0 as f32, y_0 as f32));
    while x_i < bord.1 {
        k1 = h * f(x_i, ans_h);
        k2 = h * f(x_i + h / 2.0, ans_h + k1 / 2.0);
        k3 = h * f(x_i + h / 2.0, ans_h + k2 / 2.0);
        k4 = h * f(x_i + h, ans_h + k3);
        ans_h = ans_h + (1.0 / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
        x_i += h;
        full_ans.push((x_i as f32, ans_h as f32));
    }
    x_i = bord.0;
    while x_i < bord.1 {
        k1 = (h / 2.0) * f(x_i, ans_h_2);
        k2 = (h / 2.0) * f(x_i + h / 4.0, ans_h_2 + k1 / 2.0);
        k3 = (h / 2.0) * f(x_i + h / 4.0, ans_h_2 + k2 / 2.0);
        k4 = (h / 2.0) * f(x_i + h / 4.0, ans_h_2 + k3);
        ans_h_2 = ans_h_2 + (1.0 / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
        x_i += h / 2.0;
    }
    if (ans_h - ans_h_2).abs() / 3.0 > eps {
        return eval(bord, y_0, h / 2.0, f, eps);
    } else {
        return full_ans;
    }
}
