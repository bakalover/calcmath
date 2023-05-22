use std::char::from_u32_unchecked;

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
    let mut x_i: f64 = bord.0 + 3.0 * h;
    full_ans.push((bord.0 as f32, y_0 as f32));

    if (bord.1 - bord.0) / h < 3.0 {
        return full_ans;
    }

    let (mut d1, mut d2, mut d3): (f64, f64, f64);

    let mut f_i_3 = y_0;
    let mut f_i_2 = f_i_3 + h * f(bord.0, f_i_3);
    let mut f_i_1 = f_i_2 + h * f(bord.0 + h, f_i_2);
    let mut f_i = f_i_1 + h * f(bord.0 + h + h, f_i_1);
    full_ans.push(((bord.0 + h) as f32, f_i_2 as f32));
    full_ans.push(((bord.0 + h + h) as f32, f_i_1 as f32));

    while x_i < bord.1 {
        d1 = f_i - f_i_1;
        d2 = f_i - 2.0 * f_i_1 + f_i_2;
        d3 = f_i - 3.0 * f_i_1 + 3.0 * f_i_2 - f_i_3;
        ans_h = ans_h
            + h * f_i
            + (h.powi(2) / 2.0) * d1
            + (5.0 * h.powi(3) / 12.0) * d2
            + (3.0 * h.powi(4) / 8.0) * d3;
        x_i += h;
        f_i_3 = f_i_2;
        f_i_2 = f_i_1;
        f_i_1 = f_i;
        f_i = f(x_i, ans_h);
        full_ans.push((x_i as f32, ans_h as f32));
    }

    f_i_3 = y_0;
    f_i_2 = f_i_3 + (h / 2.0) * f(bord.0, f_i_3);
    f_i_1 = f_i_2 + (h / 2.0) * f(bord.0 + h / 2.0, f_i_2);
    f_i = f_i_1 + (h / 2.0) * f(bord.0 + h / 2.0 + h / 2.0, f_i_1);
    x_i = bord.0 + 3.0 * (h / 2.0);

    while x_i < bord.1 {
        d1 = f_i - f_i_1;
        d2 = f_i - 2.0 * f_i_1 + f_i_2;
        d3 = f_i - 3.0 * f_i_1 + 3.0 * f_i_2 - f_i_3;
        ans_h_2 = ans_h_2
            + (h / 2.0) * f_i
            + (h / 2.0).powi(2) / 2.0 * d1
            + 5.0 * (h / 2.0).powi(3) / 12.0 * d2
            + 3.0 * (h / 2.0).powi(4) / 8.0 * d3;
        x_i += h / 2.0;
        f_i_3 = f_i_2;
        f_i_2 = f_i_1;
        f_i_1 = f_i;
        f_i = f(x_i, ans_h_2);
    }
    if (ans_h - ans_h_2).abs() / 3.0 > eps {
        return eval(bord, y_0, h / 2.0, f, eps);
    } else {
        return full_ans;
    }
}
