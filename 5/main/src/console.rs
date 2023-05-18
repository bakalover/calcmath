use std::io::stdin;

use draw::draw;
use func::{
    check_equal_steps, check_size, finite_mt,
    lagr::Lagr,
    newton::{self, Newton},
};
pub fn console() {
    println!("Введите число точек N >= 2");
    let N: usize;
    let N_res: Result<usize, _> = stdin()
        .lines()
        .next()
        .expect("End of input has been detected!")
        .expect("Problems while reading string")
        .parse();

    match N_res {
        Err(_) => {
            println!("Число точек - не число!");
            return;
        }
        Ok(res) => N = res,
    }
    if N <= 1 {
        println!("Недостаточно точек для корректной интерполяции!");
        return;
    }

    let mut pts: Vec<(f32, f32)> = Vec::new();

    println!("\nВведите точки в формате: x y");

    for _i in 0..N {
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
    pts.sort_by(|(a, b), (c, d)| a.partial_cmp(c).unwrap());

    let (x_args, y_args): (Vec<f32>, Vec<f32>) = pts.iter().cloned().unzip();
    match check_size(&x_args, &y_args) {
        Err(()) => {
            println!("Размеры массивов координат некорректны!");
            return;
        }
        Ok(()) => (),
    }

    let lagr = Lagr::new(x_args.clone(), y_args.clone());
    draw(
        &pts,
        |x| lagr.eval(x),
        "/home/bakalover/code/calcmath/5/out/lagr.png".to_string(),
    )
    .unwrap();

    println!("Интерполяция по Лагранжу построена.");

    match check_equal_steps(&x_args) {
        Err(()) => {
            println!("Беда с равномерностью сетки!");
            return;
        }
        Ok(()) => (),
    }
    let mt = finite_mt(&x_args, &y_args);

    println!("\nМатрица конечных разностей: ");
    for i in 0..mt.get_data().len() {
        for j in 0..mt[i].len() {
            print!("{} ", mt[i][j]);
        }
        println!("");
    }
    let newton = Newton::new(mt, x_args);

    draw(
        &pts,
        |x| newton.eval(x),
        "/home/bakalover/code/calcmath/5/out/newton.png".to_string(),
    )
    .unwrap();
    println!("Интерполяция по Ньютону построена.");
}
