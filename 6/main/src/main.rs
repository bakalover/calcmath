use std::io::stdin;

use draw::draw;
use func::{adams, runge, super_euler};

fn main() {
    println!("Выберите функцию: ");
    println!("-> 1) y` = x + y"); //e^x - x - 1
    println!("-> 2) y` = x * y"); // e^(x^2/2)
    println!("-> 3) y` = x / y"); //sqrt(c + x^2)
    let f: fn(f32) -> f32;

    let func_choice: usize;
    let func_choice_re: Result<usize, _> = stdin()
        .lines()
        .next()
        .expect("End of input has been detected!")
        .expect("Problems while reading string")
        .parse();

    match func_choice_re {
        Err(_) => {
            println!("Опция функции - не число!");
            return;
        }
        Ok(res) => func_choice = res,
    }

    match func_choice {
        1 => f = |x| x.exp() - x - 1.0,
        2 => f = |x| (x.powi(2) / 2.0).exp(),
        3 => f = |x| (1.0 + x).sqrt(),
        _ => {
            println!("Неверная опция выбора функции!");
            return;
        }
    }

    println!("Введите границы исследуемого интервала: ");
    let borders: Vec<f32> = match stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<f32>())
        .collect::<Result<Vec<f32>, _>>()
    {
        Err(_) => {
            println!("Некорректно заданные границы!");
            return;
        }
        Ok(res) => {
            if res.len() != 2 {
                println!("Границ должно быть две!");
                return;
            }
            if res[0] > res[1] {
                println!("Границы расположены в неверном порядке!");
                return;
            }
            res
        }
    };

    println!("Введите начальное значение y_0: ");
    let y_0 = match stdin().lines().next().unwrap().unwrap().parse::<f64>() {
        Err(_) => {
            println!("Значение - не число!");
            return;
        }
        Ok(res) => res,
    };

    println!("Введите шаг h: ");
    let h = match stdin().lines().next().unwrap().unwrap().parse::<f64>() {
        Err(_) => {
            println!("Шаг - не число!");
            return;
        }
        Ok(res) => res,
    };

    println!("Введите точность eps: ");
    let eps = match stdin().lines().next().unwrap().unwrap().parse::<f64>() {
        Err(_) => {
            println!("Точность - не число!");
            return;
        }
        Ok(res) => res,
    };

    let (l, r) = (borders[0], borders[1]);
    draw(
        &super_euler::eval((l as f64, r as f64), y_0, h, |x, y| x + y, eps),
        f,
        "out/eu.png".to_string(),
    );
}
