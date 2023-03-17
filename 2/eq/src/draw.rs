use core::fmt;
use std::{f64::{NEG_INFINITY, INFINITY}};

use plotters::prelude::*;

use crate::{
    calc_util::Data,
    func_util::{get_func_by_type, get_func_name},
};
pub fn draw(data: &Data) -> Result<(), Box<dyn std::error::Error>> {
    let f_max = get_max(data)?;
    let f_min = get_min(data)?;
    let root = BitMapBackend::new("/home/bakalover/code/calcmath/2/eq/out/graph.png", (640, 480))
        .into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            get_func_name(&data.func_type),
            ("sans-serif", 50).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d((data.l - 2.0) as f32..(data.r + 2.0) as f32, (f_min - 2.0) as f32..(f_max + 2.0) as f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            ((data.l * 200.0) as i32..=(data.r * 200.0) as i32)
                .map(|x| x as f32 / 200.0)
                .map(|x| (x, get_func_by_type(&data.func_type)(&(x as f64)) as f32)),
            &RED,
        ))?;
        

    chart
        .draw_series(LineSeries::new(
            (((data.l - 2.0) * 50.0) as i32..=((data.r + 2.0) * 50.0) as i32)
                .map(|x| x as f32 / 50.0)
                .map(|x| (x, 0.0)),
            &BLUE,
        ))?;
        

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}


fn get_min(data: &Data) -> Result<f64, Box<dyn std::error::Error>>{
    let mut min_val: f64 = INFINITY;
    let func_der1 = get_func_by_type(&data.func_type);
    for i in ((data.l * 1000.0) as i64)..((data.r * 1000.0) as i64) {
        if func_der1(&((i as f64) / 1000.0)).abs() <  min_val {
            min_val = func_der1(&((i as f64) / 1000.0)).abs();
        }
    }
    match min_val {
        INFINITY => return Err(Box::new(fmt::Error)),
        NEG_INFINITY => return Err(Box::new(fmt::Error)),
        _ => return Ok(min_val),
    }
}

fn get_max(data: &Data) -> Result<f64, Box<dyn std::error::Error>> {
    let mut max_val: f64 = NEG_INFINITY;
    let func_der1 = get_func_by_type(&data.func_type);
    for i in ((data.l * 1000.0) as i64)..((data.r * 1000.0) as i64) {
        if func_der1(&((i as f64) / 1000.0)).abs() > max_val {
            max_val = func_der1(&((i as f64) / 1000.0)).abs();
        }
    }
    match max_val {
        INFINITY => return Err(Box::new(fmt::Error)),
        NEG_INFINITY => return Err(Box::new(fmt::Error)),
        _ => return Ok(max_val),
    }
}