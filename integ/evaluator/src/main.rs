use std::io::{stdin, stdout, Write};

use funcs_lib::{eval, get_func, Funcs, Methods, Req};

fn main() {
    let (l, r, mut n, eps, method): (f64, f64, u64, f64, Methods);
    let (mut I_1, mut I_0) = (0.0, f64::MAX);
    let mut req: Req;
    let mut f;


    println!("\nВыберити функцию:");
    println!("A: 5e^x + sin(x)");
    println!("B: x^3 + 12x^2 + 1");
    println!("C: arctan(x) + 5x");
    println!("D: √( x^2 + x^4 )");

    match stdin()
        .lines()
        .next()
        .expect("End of input has been detected!")
        .expect("Problems while reading string")
        .as_str()
    {
        "A" => f = get_func(Funcs::A),
        "B" => f = get_func(Funcs::B),
        "C" => f = get_func(Funcs::C),
        "D" => f = get_func(Funcs::D),
        _ => {
            println!("Некорректный выбор!");    
            return;
        }
    }


    println!("\nМетод:");
    println!("A: Левые прямоугольники");
    println!("B: Правые прямоугольники");
    println!("C: Центральные прямоугольники");
    println!("D: Трапеций");
    println!("E: Симпсона");

    match stdin()
        .lines()
        .next()
        .expect("End of input has been detected!")
        .expect("Problems while reading string")
        .as_str()
    {
        "A" => method = Methods::Left,
        "B" => method = Methods::Right,
        "C" => method = Methods::Center,
        "D" => method = Methods::Trap,
        "E" => method = Methods::Simp,
        _ => {
            println!("Некорректный выбор!");
            return;
        }
    }

    println!("\nВведите интервал через пробел:");
    print!("l r: ");
    stdout().flush();
    
    let bounds: Result<Vec<f64>, _> = stdin()
        .lines()
        .next()
        .expect("End of input has been detected!")
        .expect("Problems while reading string")
        .split(" ")
        .map(|s| -> Result<f64, _> { s.to_string().parse() })
        .collect();

    match bounds {
        Err(_) => {
            println!("Одна или две границы - не числа!");
            return;
        }
        Ok(res) => {
            if res.len() != 2 {
                println!("Неправильное количество границ!");
                return;
            }
            l = res[0];
            r = res[1];
        }
    }

    print!("\nВведите число знаков точности: ");
    stdout().flush();
    
    let epsr: Result<f64, _> = stdin()
        .lines()
        .next()
        .expect("End of input has been detected!")
        .expect("Problems while reading string")
        .parse();

    match epsr {
        Err(_) => {
            println!("Точность - не число!");
            return;
        }
        Ok(res) => eps = 1.0 / ((10.0 as f64).powf(res)),
    }

    n = 4;

    req = Req {
        l: l,
        r: r,
        n: n,
        f: f,
    };

    I_0 = eval(&req, &method);
    n *= 2;
    req.n = n;
    I_1 = eval(&req, &method);

    while (I_1 - I_0).abs() > eps {
        n *= 2;
        req.n = n;
        I_0 = I_1;
        I_1 = eval(&req, &method);
    }

    println!("\nЗначение интеграла: {}\nЧисло разбиений: {}", I_1, n);
}
