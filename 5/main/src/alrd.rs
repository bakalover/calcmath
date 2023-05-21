use std::io::{stdin, stdout, Write};

use draw::draw;
use func::{check_eq_args, check_equal_steps, check_size, finite_mt, lagr::Lagr, newton::Newton, gauss::Gauss};

pub fn alrd() {
    println!("Выберите функцию: ");
    println!("-> 1) sin(sin(x))");
    println!("-> 2) 1/(1 + x^2 + x^4)");
    println!("-> 3) e^(sin(x))");
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
        1 => f = |x| x.sin().sin(),
        2 => f = |x| 1.0 / (1.0 + x.powi(2) + x.powi(4)),
        3 => f = |x| x.sin().exp(),
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

    println!("Введите число число интервалов: ");
    let n = match stdin().lines().next().unwrap().unwrap().parse::<usize>() {
        Err(_) => {
            println!("Число интервалов - не число!");
            return;
        }
        Ok(res) => res,
    };

    let (mut l, r) = (borders[0], borders[1]);
    let h = (r - l) / (n as f32);

    let mut pts: Vec<(f32, f32)> = Vec::new();

    while l <= r {
        pts.push((l, f(l)));
        l += h;
    }

    print!("Введите аргумент: ");
    stdout().flush().unwrap();
    let x: f32 = stdin()
        .lines()
        .next()
        .expect("End of input has been detected!")
        .expect("Problems while reading string")
        .parse()
        .unwrap();
    println!("");

    pts.sort_by(|(a, _), (c, _)| a.partial_cmp(c).unwrap());

    let (x_args, y_args): (Vec<f32>, Vec<f32>) = pts.iter().cloned().unzip();

    match check_size(&x_args, &y_args) {
        Err(()) => {
            println!("Размеры массивов координат некорректны!");
            return;
        }
        Ok(()) => (),
    }

    match check_eq_args(&x_args) {
        true => (),
        false => {
            println!(
                "Найдены одинаковые аргументы x\n-> Невозможно применить схемы интерполирования"
            );
            return;
        }
    }

    let lagr = Lagr::new(x_args.clone(), y_args.clone());
    draw(&pts, |x| lagr.eval(x), "out/lagr.png".to_string()).unwrap();

    println!("Интерполяция по Лагранжу построена\n-> Проверьте папку с графиком");
    println!("Значение функции: {}", lagr.eval(x));

    match check_equal_steps(&x_args) {
        Err(()) => {
            println!(
                "Беда с равномерностью сетки!\n-> Невозможно построить равномерную интерполяцию"
            );
            return;
        }
        Ok(()) => (),
    }
    let mt = finite_mt(&x_args, &y_args);

    println!("\nМатрица конечных разностей: ");
    for i in 0..mt.get_data().len() {
        for j in 0..mt[i].len() {
            print!("|{:>10.3}| ", mt[i][j]);
        }
        println!("");
    }
    let newton = Newton::new(mt.clone(), x_args.clone());

    draw(&pts, |x| newton.eval(x), "out/newton.png".to_string()).unwrap();
    println!("Интерполяция по Ньютону построена\n-> Проверьте папку с графиком");
    println!("Значение функции: {}", newton.eval(x));

     let gauss = Gauss::new(mt.clone(), x_args.clone());

    draw(&pts, |x| gauss.eval_stir(x), "out/gauss.png".to_string()).unwrap();
    println!("Интерполяция по  Стирлингу построена\n-> Проверьте папку с графиком");
    println!("Значение функции: {}", gauss.eval_stir(x));
}
