use plotters::prelude::*;

pub fn draw<F: Fn(f32) -> f32>(
    pts: &[(f32, f32)],
    f: F,
    path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let (x_args, y_args): (Vec<f32>, Vec<f32>) = pts.iter().cloned().unzip();

    let borders: ((f32, f32), (f32, f32)) = (
        (
            x_args.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
            x_args.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)),
        ),
        (
            y_args.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
            y_args.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)),
        ),
    );

    let root = BitMapBackend::new(&path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (borders.0 .0 - 2.0)..(borders.0 .1 + 2.0),
            (borders.1 .0 - 2.0)..(borders.1 .1 + 2.0),
        )?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        ((borders.0 .0 * 200.0) as i32..=(borders.0 .1 * 200.0) as i32)
            .map(|x| x as f32 / 200.0)
            .map(|x| (x, f(x))),
        &RED,
    ))?;

    chart.draw_series(PointSeries::of_element(
        pts.to_vec(),
        5,
        ShapeStyle::from(&BLUE).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord)
                + Circle::new((0, 0), size, style)
                + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 18))
        },
    ))?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
