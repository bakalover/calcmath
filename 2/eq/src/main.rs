use std::io::stdin;

use eq::calc_util::newton::calculate_newton;
use eq::calc_util::newton_multi::calculate_newton_multi;
use eq::calc_util::{calculate, Data, Methods};
use eq::console::from_console;
use eq::draw::draw;
use eq::file::from_file;
use eq::func_util::Funcs;
use eq::multi::multi;
fn main() {
    // let data = Data {
    //     method: Methods::Simpl,
    //     func_type: Funcs::Log,
    //     l: 0.1,
    //     r: 5.0,
    //     eps: 1.0 / ((10 as u64).pow(6) as f64),
    // };
    // match draw(&data) {
    //     Ok(_) => println!("Проверьте ваш график в папке out"),
    //     Err(_) => println!("Невозможно построить график!"),
    // }

    // match calculate(&data) {
    //     Ok(out) => println!(
    //         "Приближенный Корень: {}\nЧисло итераций: {}\nЗначение функции в корне: {}",
    //         out.ans, out.iters, out.f
    //     ),
    //     Err(err) => println!("{}", err.0),
    // }


    println!("\nВыберете Конфигурацию: \n1 - Консоль\n2 - Файл\n3 - Системы");

    let mut choice = String::new();

    stdin().read_line(&mut choice).expect("IO problems");

    match choice.as_str().trim() {
        "1" => match from_console() {
            Ok(()) => println!("Корректное завершение работы.",),
            Err(msg) => println!("Некорректное завершение работы:\n{}", msg.0),
        },

        "2" => match from_file() {
            Ok(()) => println!("Корректное завершение работы."),
            Err(msg) => println!("Некорректное завершение работы:\n{}", msg.0),
        },

        "3" => match multi() {
            Ok(()) => println!("Корректное завершение работы."),
            Err(msg) => println!("Некорректное завершение работы:\n{}", msg.0),
        },

        _ => println!("Некорректный ввод!"),
    }
}
