use funcs::poly;

fn main() {
    let a:Vec<(f32, f32)> = vec![(1.0,1.0),(1.5,1.5),(5.0,7.0),(8.0,20.0),(-10.0,-3.0)];
    let linear = match poly(a.as_slice(), funcs::Type::Cube){
        Ok(val) => val,
        Err(err) =>return,
    };
    println!("{} {} {} {}",linear[0],linear[1], linear[2], linear[3]); 
}
