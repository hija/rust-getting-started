use rand::distributions::{Normal, Distribution};
use plotters::prelude::*;

fn generate_dataset(length: usize, init_value: f64) -> Vec<f64> {
    let mut dataset:Vec<f64> = vec![0.0; length];
    dataset[0] = init_value;

    let normal = Normal::new(0.0, 1.0);
    let mut thread = rand::thread_rng();
    
    for i in 1..length{
        dataset[i] = dataset[i-1] + normal.sample(&mut thread);
    }
    return dataset;
}

fn exponential_smoothing(dataset: Vec<f64>, alpha: f64) -> Vec<f64> {
    let mut smoothed_dataset:Vec<f64> = vec![0.0; dataset.len()];
    smoothed_dataset[0] = dataset[0];
    for i in 1..dataset.len(){
        smoothed_dataset[i] = (dataset[i] * alpha) + (smoothed_dataset[i-1] * (1.0-alpha));
    }
    return smoothed_dataset;
}

fn plot_dataset(dataset: Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {

    //let dataset_min = dataset.iter().copied().fold(f64::NAN, f64::min);
    let dataset_max = dataset.iter().copied().fold(f64::NAN, f64::max);
    let smoothed_dataset = exponential_smoothing(dataset.clone(), 2.0 / (dataset.len()  as f64 + 1.0));

    let root = BitMapBackend::new("timeseries.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Timeseries", ("sans-serif", 50).into_font())
        .margin::<u32>(5)
        .x_label_area_size::<u32>(30)
        .y_label_area_size::<u32>(30)
        .build_cartesian_2d(0.0..(dataset.len()-1) as f64, 0.0..dataset_max)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..dataset.len()).map(|x| (x as f64, dataset[x] as f64)),
            &RED,
        ))?
        .label("Timeseries")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    

    chart
    .draw_series(LineSeries::new(
        (0..dataset.len()).map(|x| (x as f64, smoothed_dataset[x] as f64)),
        &BLUE,
    ))?
    .label("Exponential Smoothed Timeseries")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));



    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn main() {
    plot_dataset(generate_dataset(500, 5.0)).unwrap();
}
