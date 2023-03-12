use std::io::stdin;

use eq::calc_util::{calculate, Data, Methods};
use eq::console::from_console;
use eq::file::from_file;
use eq::func_util::{get_func_by_type, Funcs};
fn main() {
    let data = Data {
        method: Methods::Simpl,
        func_type: Funcs::Log,
        l: 0.5,
        r: 2.0,
        eps: 1.0 / ((10 as u64).pow(6) as f64),
    };
    match calculate(&data) {
        Ok(out) => println!(
            "Приближенный Корень: {}\nЧисло итераций: {}\nЗначение функции в корне: {}",
            out.ans, out.iters, out.f
        ),
        Err(err) => println!("{}", err.0),
    }

    println!("\nВыберете Конфигурацию: \n1 - Консоль, 2 - Файл\n");

    let mut choice = String::new();

    stdin().read_line(&mut choice).expect("IO problems");

    match choice.as_str().trim() {
        "1" => match from_console() {
            Ok(msg) => println!("{}", msg),
            Err(err) => println!("{}", err),
        },
        "2" => match from_file() {
            Ok(msg) => println!("{}", msg),
            Err(err) => println!("Некорректный ввод"),
        },
        _ => println!("Некорректный ввод!"),
    }
}
