use std::{
    error,
    fmt::Error,
    io::{self, stdin, stdout, BufRead},
    str::FromStr,
};

use crate::func_util::*;

pub fn from_console() -> Result<String, Box<dyn error::Error>> {
    let mut choice = String::new();

    println!("Выберете функцию: 1, 2, 3, 4\n");
    println!("1) {}", get_func_name(Funcs::Log));
    println!("2) {}", get_func_name(Funcs::Poly));
    println!("3) {}", get_func_name(Funcs::Sinh));
    println!("4) {}", get_func_name(Funcs::PolySin));

    stdin().read_line(&mut choice).expect("IO problems");

    match choice.as_str().trim() {
        "1" | "2" | "3" => println!("Выш выбор: {}", choice),
        _ => println!("Некорректный ввод!"),
    }

    println!("Введите границы:\n");
    /*let arr: Vec<f64> = stdin()
        .lines()
        .next()
        .expect("NO String")?
        .split(' ')
        .map(|x| -> Result<f64, Box<dyn FromStr::Err>> { str::parse(x)? })
        .map(|x|->Option<f64>{x.ok()})
        .map(|x| -> f64{match x {
        None=>0.0,
        Some(k)=>k,
        }})
        .collect();
    print!("{} {}",arr[0],arr[1] );*/
    println!("l: ");
    let l: f64 = stdin()
        .lines()
        .next()
        .expect("Lock on stdio")
        .expect("IO problems")
        .trim()
        .parse()?;
    println!("r: ");
    let r: f64 = stdin()
        .lines()
        .next()
        .expect("Lock on stdio")
        .expect("IO problems")
        .trim()
        .parse()?;
    if (l >= r) {
        return Err(Box::new(Error));
    }
    println!("\nВведите точность:\neps: ");
    let eps: u32 = stdin()
        .lock()
        .lines()
        .next()
        .expect("Lock on stdio")
        .expect("IO problems")
        .trim()
        .parse()?;

    Ok(("\nРабота завершена, проверте папку с графиком\n").to_string())
}
