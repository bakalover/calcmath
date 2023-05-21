use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use draw::draw;
use func::{gauss::Gauss, lagr::Lagr, newton::Newton, *};

pub fn file() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);
    let mut pts = Vec::<(f32, f32)>::new();
    let mut x = 0.0;
    for line in reader.lines() {
        let pt: Vec<f32> = line
            .unwrap()
            .split(' ')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        if pt.len() >= 2 {
            pts.push((pt[0], pt[1]));
        } else {
            x = pt[0];
        }
    }
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
