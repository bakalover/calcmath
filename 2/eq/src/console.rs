use std::{
    error,
    fmt::Error,
    io::{stdin, BufRead},
};

use crate::{func_util::*, calc_util::{Data, Methods, calculate}, draw::draw};

pub struct RuntimeError(pub String);

pub fn from_console() -> Result<String, RuntimeError> {
    let mut choice_func = String::new();
    let mut choice_method = String::new();
    let method: Methods;
    let func_type: Funcs;

    println!("Выберете функцию: 1, 2, 3, 4\n");
    println!("1) {}", get_func_name(&Funcs::Log));
    println!("2) {}", get_func_name(&Funcs::Poly));
    println!("3) {}", get_func_name(&Funcs::Sinh));
    println!("4) {}", get_func_name(&Funcs::PolySin));

    stdin().read_line(&mut choice_func);

    match choice_func.as_str().trim() {
        "1"=>func_type = Funcs::Log,
        "2"=>func_type = Funcs::Poly,
        "3"=>func_type = Funcs::Sinh,
        "4"=>func_type = Funcs::PolySin,
        _ => return Err(RuntimeError("Неверная опция выбора функции!".to_string())),
    }

    println!("Выберете Способ: 1, 2, 3\n");
    println!("1) Бинарный поиск");
    println!("2) Метод Ньютона");
    println!("3) Метод простой Итерации");

    stdin().read_line(&mut choice_method);

    match choice_method.as_str().trim() {
        "1"=>method = Methods::Bin,
        "2"=>method = Methods::Newton,
        "3"=>method = Methods::Simpl,
        _ => return Err(RuntimeError("Неверная опция выбора метода!".to_string())),
    }

    println!("Введите границы:\n");

    // let arr: Vec<f64> = stdin()
    //     .lines()
    //     .next()
    //     .expect("NO String")?
    //     .split(' ')
    //     .map(|x| -> Result<f64, Box<dyn FromStr::Err>> { str::parse(x)? })
    //     .map(|x|->Option<f64>{x.ok()})
    //     .map(|x| -> f64{match x {
    //     None=>0.0,
    //     Some(k)=>k,
    //     }})
    //     .collect();
    // print!("{} {}",arr[0],arr[1] );

    let l: f64;
    let r: f64;
    let eps: u32;

    println!("l: ");

    match stdin()
        .lines()
        .next()
        .expect("Lock on stdio")
        .expect("IO problems")
        .trim()
        .parse()
    {
        Ok(res) => l = res,
        Err(_) => return Err(RuntimeError("Левая граница - не число!".to_string())),
    }

    println!("r: ");

    match stdin()
        .lines()
        .next()
        .expect("Lock on stdio")
        .expect("IO problems")
        .trim()
        .parse()
    {
        Ok(res) => r = res,
        Err(_) => return Err(RuntimeError("Правая граница - не число!".to_string())),
    }

    if l >= r {
        return Err(RuntimeError("Левая граница больше правой!".to_string()));
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
        Err(_) => return Err(RuntimeError("Точность - не число!".to_string())),
    }

    let data = Data{
        method: method,
        func_type: func_type,
        l: l,
        r: r,
        eps: 1.0 / ((10 as u64).pow(eps) as f64)
    };

    match calculate(&data) {
        Ok(out) => println!(
            "Приближенный Корень: {}\nЧисло итераций: {}\nЗначение функции в корне: {}",
            out.ans, out.iters, out.f
        ),
        Err(err) => return Err(RuntimeError(err.0)),
    }

    match draw(&data) {
        Ok(_) => return Ok("Проверьте ваш график в папке out".to_string()),
        Err(_) => return Err(RuntimeError("Невозможно построить график!".to_string())),
    }

}
