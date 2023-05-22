use std::io::stdin;

use draw::draw;
use func::{adams, runge, super_euler};

fn main() {
    println!("Выберите функцию: ");
    println!("-> 1) y` = x + y"); //e^x - x - 1
    println!("-> 2) y` = x * y"); // e^(x^2/2)
    println!("-> 3) y` = x / y"); //sqrt(c + x^2)
    let f: fn(f32) -> f32;
    let f_base: fn(f64, f64) -> f64;

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
        1 => {
            f = |x| x.exp() - x - 1.0;
            f_base = |x, y| x + y;
        }
        2 => {
            f = |x| (x.powi(2) / 2.0).exp();
            f_base = |x, y| x * y;
        }
        3 => {
            f = |x| (1.0 + x).sqrt();
            f_base = |x, y| x / y;
        }
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

    println!("Выберите метод: ");
    println!("-> 1) Супер-Эйлер"); //e^x - x - 1
    println!("-> 2) Рунге-Кутта 4-й степени"); // e^(x^2/2)
    println!("-> 3) Адамса"); //sqrt(c + x^2)

    let method_choice = match stdin().lines().next().unwrap().unwrap().parse::<usize>() {
        Err(_) => {
            println!("Опция метода - не число!");
            return;
        }
        Ok(res) => res,
    };

    let (l, r) = (borders[0], borders[1]);

    match method_choice {
        1 => draw(
            &super_euler::eval((l as f64, r as f64), y_0, h, f_base, eps),
            f,
            "out/eu.png".to_string(),
        )
        .unwrap(),
        2 => draw(
            &runge::eval((l as f64, r as f64), y_0, h, f_base, eps),
            f,
            "out/rung.png".to_string(),
        )
        .unwrap(),
        3 => draw(
            &adams::eval((l as f64, r as f64), y_0, h, f_base, eps),
            f,
            "out/adm.png".to_string(),
        )
        .unwrap(),
        _ => {
            println!("Неверная опция выбора метотда!");
            return;
        }
    }
    println!("Приближённое решение построено \n->Проверьте папку с графиком");
}
