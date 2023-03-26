use std::io::stdin;
use eq::console::from_console;
use eq::file::from_file;
use eq::multi::multi;
fn main() {
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
