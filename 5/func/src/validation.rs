pub fn check_size(x_arr: &[f32], y_arr: &[f32]) -> Result<(), ()> {
    if x_arr.len() <= 1 {
        return Err(());
    }
    if y_arr.len() <= 1 {
        return Err(());
    }
    if x_arr.len() != y_arr.len() {
        return Err(());
    }
    Ok(())
}

pub fn check_equal_steps(x_arr: &[f32]) -> Result<(), ()> {
    let h = x_arr[1] - x_arr[0];
    for i in 2..x_arr.len() - 1 {
        if x_arr[i] - x_arr[i - 1] != h {
            return Err(());
        }
    }
    Ok(())
}
