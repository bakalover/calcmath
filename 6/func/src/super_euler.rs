pub fn eval<F: Fn(f64, f64) -> f64>(bord: (f64, f64), y_0: f64, h: f64, f: F, eps: f64) -> f64 {
    let mut ans_h:f64 = y_0;
    let mut ans_h_2:f64 = y_0;
    let mut x_i:f64 = bord.0;
    while x_i < bord.1 {
        ans_h = ans_h + (h / 2.0) * (f(x_i, ans_h) + f(x_i, ans_h + h * f(x_i, ans_h)));
        x_i += h;
    }
    x_i = bord.0;
    while x_i < bord.1 {
        ans_h_2 =
            ans_h_2 + (h / 4.0) * (f(x_i, ans_h_2) + f(x_i, ans_h_2 + (h / 2.0) * f(x_i, ans_h_2)));
        x_i += h / 2.0;
    }
    if (ans_h - ans_h_2).abs() / 3.0 > eps {
        return eval(bord, y_0, h / 2.0, f, eps);
    } else {
        return ans_h_2;
    }
}
