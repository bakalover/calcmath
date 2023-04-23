use std::io::{stdout, Write};

pub fn get_emp<F: Fn(f32) -> f32>(arr: &[(f32, f32)], f: F) -> (Vec<(f32, f32)>, f32) {
    let mut sigma: f32 = 0.0;
    let mut y_i;
    let mut empir: Vec<(f32, f32)> = Vec::new();

    for pt in arr {
        y_i = f(pt.0);
        empir.push((y_i, y_i - pt.1));
        sigma += (y_i - pt.1).powi(2);
    }

    (empir, (sigma / (arr.len() as f32)).sqrt())
}

pub fn print_metrics(metrics: &(Vec<(f32, f32)>, f32)) -> () {
    println!("  ");
    print!("Отклонение: ");
    stdout().flush().unwrap();
    println!("{}", metrics.1);
    println!("Эмпирическая зависимость: ");
    print!("y_i: ");
    stdout().flush().unwrap();
    for emp in &metrics.0 {
        print!(" {} ", emp.0);
    }
    stdout().flush().unwrap();
    print!("\ne_i: ");
    stdout().flush().unwrap();
    for emp in &metrics.0 {
        print!(" {} ", emp.1);
    }
    stdout().flush().unwrap();
    println!("  ");
}
