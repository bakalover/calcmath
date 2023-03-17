use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::{
    calc_util::{calculate, Data, Methods},
    draw::draw,
    func_util::Funcs,
};

pub struct FileError(pub String);

pub fn from_file() -> Result<(), FileError> {
    let method: Methods;
    let func_type: Funcs;

    let file = match File::open("/home/bakalover/code/calcmath/2/eq/in/hello.txt") {
        Ok(file) => file,
        Err(_) => return Err(FileError("Невозможно открыть файл!".to_string())),
    };

    let reader = BufReader::new(file);

    let args: Vec<f64> = match reader
        .lines()
        .map(|line| line.unwrap().parse::<f64>())
        .collect()
    {
        Ok(vec) => vec,
        Err(_) => {
            return Err(FileError("Некорректные аргументы!".to_string()));
        }
    };

    if args.len() < 5 {
        return Err(FileError("Мало аргументов!".to_string()));
    }

    match args[0] as i64 {
        1 => func_type = Funcs::Log,
        2 => func_type = Funcs::Poly,
        3 => func_type = Funcs::Sinh,
        4 => func_type = Funcs::PolySin,
        _ => {
            return Err(FileError(
                "Неверная опция выбора функции в файле!".to_string(),
            ))
        }
    }

    match args[1] as i64 {
        1 => method = Methods::Bin,
        2 => method = Methods::Newton,
        3 => method = Methods::Simpl,
        _ => {
            return Err(FileError(
                "Неверная опция выбора метода в файле!".to_string(),
            ))
        }
    }

    let l: f64 = args[2];
    let r: f64 = args[3];
    let eps: u32 = args[4] as u32;

    if l >= r {
        return Err(FileError("Левая граница больше правой!".to_string()));
    }

    let data = Data {
        method: method,
        func_type: func_type,
        l: l,
        r: r,
        eps: 1.0 / ((10 as u64).pow(eps) as f64),
    };

    match draw(&data) {
        Ok(_) => println!("Проверьте ваш график в папке out"),
        Err(_) => println!("Невозможно построить график!"),
    }

    match calculate(&data) {
        Ok(out) => {
            println!(
                "Приближенный Корень: {}\nЧисло итераций: {}\nЗначение функции в корне: {}",
                out.ans, out.iters, out.f
            );
            return Ok(());
        }
        Err(err) => return Err(FileError(err.0)),
    }

    // match draw(&data) {
    //     Ok(_) => return Ok("Проверьте ваш график в папке out".to_string()),
    //     Err(_) => return Err(FileError("Невозможно построить график!".to_string())),
    // }
}
