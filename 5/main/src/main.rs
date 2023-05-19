mod console;
mod file;
mod alrd;

use std::io::stdin;

use console::console;
use file::file;
use alrd::alrd;
fn main() {
    println!("Выберите конфигурацию:");
    println!("-> 1) Консоль");
    println!("-> 2) Файл");
    println!("-> 3) Заготовленные функции");

    let conf: usize;
    let conf_res: Result<usize, _> = stdin()
        .lines()
        .next()
        .expect("End of input has been detected!")
        .expect("Problems while reading string")
        .parse();

    match conf_res {
        Err(_) => {
            println!("Неверная конфигурация!");
            return;
        }
        Ok(res) => conf = res,
    }

    println!("");

    match conf {
        1 => console(),
        2 => file(),
        3 => alrd(),
        _ => {
            println!("Неверная конфигурация!");
            return;
        }
    }
}
