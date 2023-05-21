use std::io::{stdin, stdout, Write};

use draw::draw;
use func::{check_eq_args, check_equal_steps, check_size, finite_mt, lagr::Lagr, newton::Newton, gauss::Gauss};
pub fn console() {
    println!("Введите число точек N >= 2");
    let pts_numb: usize;
    let pts_numb_res: Result<usize, _> = stdin()
        .lines()
        .next()
        .expect("End of input has been detected!")
        .expect("Problems while reading string")
        .parse();

    match pts_numb_res {
        Err(_) => {
            println!("Число точек - не число!");
            return;
        }
        Ok(res) => pts_numb = res,
    }
    if pts_numb <= 1 {
        println!("Недостаточно точек для корректной интерполяции!");
        return;
    }

    let mut pts: Vec<(f32, f32)> = Vec::new();

    println!("\nВведите точки в формате: x y");

    for _i in 0..pts_numb {
        let pt: Result<Vec<f32>, _> = stdin()
            .lines()
            .next()
            .expect("End of input has been detected!")
            .expect("Problems while reading string")
            .split(" ")
            .map(|s| -> Result<f32, _> { s.to_string().parse() })
            .collect();

        match pt {
            Err(_) => {
                println!("Одна или две координаты - не числа!");
                return;
            }
            Ok(res) => {
                if res.len() != 2 {
                    println!("У нас двухмерное измерение!");
                    return;
                }
                pts.push((res[0], res[1]));
            }
        }
    }

    print!("Введите аргумент: ");
    stdout().flush().unwrap();
    let x = stdin()
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
