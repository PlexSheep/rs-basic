use std::io::ErrorKind;

use plotters::prelude::*;

fn main() {
    if let Err(e) = std::fs::create_dir("output") {
        if e.kind() != ErrorKind::AlreadyExists {
            eprintln!("could not create the output directory: {e}");
            std::process::exit(1)
        }
    }
    println!("generating coordinate system");
    coordinate_system();
    println!("generating histogram");
    histo().expect("histogram failed");
    println!("generating area chart");
    area().expect("area failed");
    println!("generating timed area chart");
    time_area().expect("time area failed");

    println!("all done!")
}

fn coordinate_system() {
    let root_area = BitMapBackend::new("output/coords.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Line Plot Demo", ("sans-serif", 40))
        .build_cartesian_2d(-10..10, 0..100)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new((-10..=10).map(|x| (x, x * x)), &GREEN))
        .unwrap();
}

fn histo() -> Result<(), Box<dyn std::error::Error>> {
    const OUT_FILE_NAME: &str = "output/histogram.png";
    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    let data = [
        0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3,
    ];

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|x: &u32| (*x, 1))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())
}

fn area() -> Result<(), Box<dyn std::error::Error>> {
    const OUT_FILE_NAME: &str = "output/area.png";
    use rand::SeedableRng;
    use rand_distr::{Distribution, Normal};
    use rand_xorshift::XorShiftRng;

    let data: Vec<_> = {
        let norm_dist = Normal::new(500.0, 100.0).unwrap();
        let mut x_rand = XorShiftRng::from_seed(*b"MyFragileSeed345");
        let x_iter = norm_dist.sample_iter(&mut x_rand);
        x_iter
            .filter(|x| *x < 1500.0)
            .take(100)
            .zip(0..)
            .map(|(x, b)| x + (b as f64).powf(1.2))
            .collect()
    };

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .caption("Area Chart Demo", ("sans-serif", 40))
        .build_cartesian_2d(0..(data.len() - 1), 0.0..1500.0)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(
        AreaSeries::new(
            (0..).zip(data.iter()).map(|(x, y)| (x, *y)),
            0.0,
            RED.mix(0.2),
        )
        .border_style(RED),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    Ok(())
}

fn time_area() -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;

    use chrono::{TimeZone, Utc};

    const OUT_FILE_NAME: &str = "output/timed_data.png";
    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption(
            "Monthly Average Temperate in Salt Lake City, UT",
            ("sans-serif", 40),
        )
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Right, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(
            (Utc.ymd(2010, 1, 1)..Utc.ymd(2018, 12, 1)).monthly(),
            14.0..104.0,
        )?
        .set_secondary_coord(
            (Utc.ymd(2010, 1, 1)..Utc.ymd(2018, 12, 1)).monthly(),
            -10.0..40.0,
        );

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_labels(30)
        .max_light_lines(4)
        .y_desc("Average Temp (F)")
        .draw()?;
    chart
        .configure_secondary_axes()
        .y_desc("Average Temp (C)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        TIME_DATA.iter().map(|(y, m, t)| (Utc.ymd(*y, *m, 1), *t)),
        &BLUE,
    ))?;

    chart.draw_series(
        TIME_DATA
            .iter()
            .map(|(y, m, t)| Circle::new((Utc.ymd(*y, *m, 1), *t), 3, BLUE.filled())),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

const TIME_DATA: [(i32, u32, f64); 12 * 9] = [
    (2010, 1, 32.4),
    (2010, 2, 37.5),
    (2010, 3, 44.5),
    (2010, 4, 50.3),
    (2010, 5, 55.0),
    (2010, 6, 70.0),
    (2010, 7, 78.7),
    (2010, 8, 76.5),
    (2010, 9, 68.9),
    (2010, 10, 56.3),
    (2010, 11, 40.3),
    (2010, 12, 36.5),
    (2011, 1, 28.8),
    (2011, 2, 35.1),
    (2011, 3, 45.5),
    (2011, 4, 48.9),
    (2011, 5, 55.1),
    (2011, 6, 68.8),
    (2011, 7, 77.9),
    (2011, 8, 78.4),
    (2011, 9, 68.2),
    (2011, 10, 55.0),
    (2011, 11, 41.5),
    (2011, 12, 31.0),
    (2012, 1, 35.6),
    (2012, 2, 38.1),
    (2012, 3, 49.1),
    (2012, 4, 56.1),
    (2012, 5, 63.4),
    (2012, 6, 73.0),
    (2012, 7, 79.0),
    (2012, 8, 79.0),
    (2012, 9, 68.8),
    (2012, 10, 54.9),
    (2012, 11, 45.2),
    (2012, 12, 34.9),
    (2013, 1, 19.7),
    (2013, 2, 31.1),
    (2013, 3, 46.2),
    (2013, 4, 49.8),
    (2013, 5, 61.3),
    (2013, 6, 73.3),
    (2013, 7, 80.3),
    (2013, 8, 77.2),
    (2013, 9, 68.3),
    (2013, 10, 52.0),
    (2013, 11, 43.2),
    (2013, 12, 25.7),
    (2014, 1, 31.5),
    (2014, 2, 39.3),
    (2014, 3, 46.4),
    (2014, 4, 52.5),
    (2014, 5, 63.0),
    (2014, 6, 71.3),
    (2014, 7, 81.0),
    (2014, 8, 75.3),
    (2014, 9, 70.0),
    (2014, 10, 58.6),
    (2014, 11, 42.1),
    (2014, 12, 38.0),
    (2015, 1, 35.3),
    (2015, 2, 45.2),
    (2015, 3, 50.9),
    (2015, 4, 54.3),
    (2015, 5, 60.5),
    (2015, 6, 77.1),
    (2015, 7, 76.2),
    (2015, 8, 77.3),
    (2015, 9, 70.4),
    (2015, 10, 60.6),
    (2015, 11, 40.9),
    (2015, 12, 32.4),
    (2016, 1, 31.5),
    (2016, 2, 35.1),
    (2016, 3, 49.1),
    (2016, 4, 55.1),
    (2016, 5, 60.9),
    (2016, 6, 76.9),
    (2016, 7, 80.0),
    (2016, 8, 77.0),
    (2016, 9, 67.1),
    (2016, 10, 59.1),
    (2016, 11, 47.4),
    (2016, 12, 31.8),
    (2017, 1, 29.4),
    (2017, 2, 42.4),
    (2017, 3, 51.7),
    (2017, 4, 51.7),
    (2017, 5, 62.5),
    (2017, 6, 74.8),
    (2017, 7, 81.3),
    (2017, 8, 78.1),
    (2017, 9, 65.7),
    (2017, 10, 52.5),
    (2017, 11, 49.0),
    (2017, 12, 34.4),
    (2018, 1, 38.1),
    (2018, 2, 37.5),
    (2018, 3, 45.4),
    (2018, 4, 54.6),
    (2018, 5, 64.0),
    (2018, 6, 74.9),
    (2018, 7, 82.5),
    (2018, 8, 78.1),
    (2018, 9, 71.9),
    (2018, 10, 53.2),
    (2018, 11, 39.7),
    (2018, 12, 33.6),
];
