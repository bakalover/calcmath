pub fn get_emp<F: Fn(f32) -> f32>(arr: &[(f32, f32)], f: F) -> (Vec<(f32, f32)>, f32) {
    let mut sigma: f32 = 0.0;
    let mut y_i;
    let mut empir: Vec<(f32, f32)> = Vec::new();

    for pt in arr {
        y_i = f(pt.0);
        empir.push((y_i, y_i - pt.1));
        sigma += (y_i-pt.1).powi(2);
    }

    (empir,(sigma/arr.len() as f32).sqrt())

}
