
use funcs_lib::Req;

fn main() {
    let a = Req {
        l: 0.0,
        r: 1.0,
        n: 100000,
        eps: 5.0,
        f: |x| -> f32 { x * x },
    };
    match funcs_lib::center_c(&a) {
        Err(_) => println!("beda"),
        Ok(ans) => println!("{}", ans),
    }
}
