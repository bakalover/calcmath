use funcs::poly;

fn main() {
    let a:Vec<(f32, f32)> = vec![(1.0,1.0),(1.5,1.5),(5.0,7.0),(8.0,20.0),(-10.0,-3.0)];
    let linear = match poly(a.as_slice(), 4){
        Ok(val) => val,
        Err(err) =>return,
    };
    for i in linear{
        println!("{}",i);
    }
}
