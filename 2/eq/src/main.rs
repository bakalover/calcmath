use std::{
    io::{BufRead, stdin, self}, error,
};
use crate::{util::*, calc::{Data, Methods, calculate}};

pub mod calc;
pub mod draw;
pub mod util;

fn main() {
    /*let data = Data{
        method: Methods::Bin,
        func: Funcs::Poly,
        a: -2.0,
        b: 0.0,
    };
    println!("{}",calculate(&data));*/

    println!("\nВыберете Конфигурацию: \n1 - Консоль, 2 - Файл\n");

    let mut choice = String::new();

    stdin().read_line(&mut choice).expect("IO problems");

    match choice.as_str().trim() {
        "1"  => match from_console(){
            Ok(msg) => println!("{}",msg),
            Err(err) => println!("{}", err),
        },
        "2"  => match from_file(){
            Ok(msg) => println!("{}",msg),
            Err(err) => println!("Некорректный ввод"),
        },
        _ => println!("Некорректный ввод!"),
    }
}


fn from_file()-> Result<String, io::Error>{
    /*crate::draw::draw().unwrap();
    let reader = BufReader::new(File::open("hello.txt").unwrap());
    let mut x: Vec<i64> = reader.lines().map(|line| line.unwrap().parse::<i64>().unwrap()).collect();
    print!("{}, {}",x[0],x[1]);
    let a: Option<i32>;*/
    Ok(("\nРабота завершена, проверте папку с графиком\n").to_string())
}

fn from_console() -> Result<String, Box<dyn error::Error>>{
    let mut choice = String::new();

    println!("Выберете функцию: 1, 2, 3\n");
    println!("{}",get_func_name(Funcs::Log));
    println!("{}",get_func_name(Funcs::Poly));
    println!("{}\n",get_func_name(Funcs::Sin));

    stdin().read_line(&mut choice).expect("IO problems");
    
    match choice.as_str().trim() {
        "1" | "2" | "3" => println!("Выш выбор: {}",choice),
        _ => println!("Некорректный ввод!"),
    }
    
    println!("Введите границы:\n");

    let l: f64 = stdin().lock().lines().next().expect("Lock on stdio").expect("IO problems").trim().parse()?;
    let r: f64 = stdin().lock().lines().next().expect("Lock on stdio").expect("IO problems").trim().parse()?;

    println!("\nВведите точность:\n");

    let eps: i32 = stdin().lock().lines().next().expect("Lock on stdio").expect("IO problems").trim().parse()?;

    Ok(("\nРабота завершена, проверте папку с графиком\n").to_string())
}