use std::io::stdin;

use crate::{calc_util::newton_multi::calculate_newton_multi, draw::draw_multi};

pub struct MultiRuntimeError(pub String);

pub fn multi() -> Result<(), MultiRuntimeError> {
    let mut choice_sys = String::new();
    let sys_type;

    println!("Выберете Систему: 1, 2\n");
    println!("1.\nsin(x - y) = 0\ncos(x + y) = 0\n");
    println!("2.\nx^2 + y^2 = 30\ny = sin(x/2)");

    stdin().read_line(&mut choice_sys);

    match choice_sys.as_str().trim() {
        "1" => sys_type = 1,
        "2" => sys_type = 2,
        _ => {
            return Err(MultiRuntimeError(
                "Неверная опция выбора системы!".to_string(),
            ))
        }
    }

    println!("Введите приближения:\n");
    let x: f64;
    let y: f64;
    let eps: u32;

    println!("x: ");

    match stdin()
        .lines()
        .next()
        .expect("Lock on stdio")
        .expect("IO problems")
        .trim()
        .parse()
    {
        Ok(res) => x = res,
        Err(_) => return Err(MultiRuntimeError("Левая граница - не число!".to_string())),
    }

    println!("y: ");

    match stdin()
        .lines()
        .next()
        .expect("Lock on stdio")
        .expect("IO problems")
        .trim()
        .parse()
    {
        Ok(res) => y = res,
        Err(_) => return Err(MultiRuntimeError("Левая граница - не число!".to_string())),
    }

    println!("\nВведите точность:\neps: ");

    match stdin()
        .lines()
        .next()
        .expect("Lock on stdio")
        .expect("IO problems")
        .trim()
        .parse()
    {
        Ok(res) => eps = res,
        Err(_) => return Err(MultiRuntimeError("Точность - не число!".to_string())),
    }
    calculate_newton_multi(x, y, eps, sys_type);
    Ok(())
}
