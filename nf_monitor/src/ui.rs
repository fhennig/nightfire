use crate::monitor::MonitorData;
use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_piston::{draw_piston_window, PistonBackend};
use std::sync::{Arc, Mutex};

const FPS: u32 = 30;
const LENGTH: u32 = 20;
const N_DATA_POINTS: usize = (FPS * LENGTH) as usize;

pub fn create_onset_plot<'a, 'b>(
    root: &'a DrawingArea<PistonBackend, Shift>,
    data: &MonitorData,
) -> Result<
    (),
    DrawingAreaErrorKind<<PistonBackend<'a, 'b> as plotters::prelude::DrawingBackend>::ErrorType>,
> {
    let mut cc = ChartBuilder::on(&root)
        .margin(10)
        .caption("Onsets", ("sans-serif", 20))
        .x_label_area_size(20)
        .y_label_area_size(25)
        .build_cartesian_2d(0..N_DATA_POINTS as u32, 0f32..10f32)?;

    cc.configure_mesh().draw()?;

    cc.draw_series(LineSeries::new(
        (0..).zip(data.onset_stddevs.iter()).map(|(a, b)| (a, *b)),
        &Palette99::pick(2),
    ))?
    .label("Onset Stddev")
    .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(2)));
    cc.draw_series(LineSeries::new(
        (0..).zip(data.onset_means.iter()).map(|(a, b)| (a, *b)),
        &Palette99::pick(1),
    ))?
    .label("Onset Mean")
    .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(1)));
    cc.draw_series(LineSeries::new(
        (0..).zip(data.onset_scores.iter()).map(|(a, b)| (a, *b)),
        &Palette99::pick(0),
    ))?
    .label("Onset Score")
    .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(0)));
    cc.draw_series(LineSeries::new(
        (0..).zip(data.onset_threshold.iter()).map(|(a, b)| (a, *b)),
        &Palette99::pick(3),
    ))?
    .label("Onset Threshold")
    .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(3)));

    cc.configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}

pub fn create_intensity_plot<'a, 'b>(
    root: &'a DrawingArea<PistonBackend, Shift>,
    data: &MonitorData,
) -> Result<
    (),
    DrawingAreaErrorKind<<PistonBackend<'a, 'b> as plotters::prelude::DrawingBackend>::ErrorType>,
> {
    let mut cc = ChartBuilder::on(&root)
        .margin(10)
        .caption("Intensity", ("sans-serif", 20))
        .x_label_area_size(20)
        .y_label_area_size(25)
        .build_cartesian_2d(0..N_DATA_POINTS as u32, 0f32..1f32)?;

    cc.configure_mesh().draw()?;

    //Total
    cc.draw_series(LineSeries::new(
        (0..)
            .zip(data.total_intensities.iter())
            .map(|(a, b)| (a, *b)),
        &Palette99::pick(1),
    ))?
    .label("Total")
    .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(1)));
    // Highs
    cc.draw_series(LineSeries::new(
        (0..)
            .zip(data.highs_intensities.iter())
            .map(|(a, b)| (a, *b)),
        &Palette99::pick(3),
    ))?
    .label("Highs")
    .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(3)));
    // Bass
    cc.draw_series(LineSeries::new(
        (0..)
            .zip(data.bass_intensities.iter())
            .map(|(a, b)| (a, *b)),
        &Palette99::pick(0),
    ))?
    .label("Bass")
    .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(0)));

    cc.configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}

pub fn create_spikiness_plot<'a, 'b>(
    root: &'a DrawingArea<PistonBackend, Shift>,
    data: &MonitorData,
) -> Result<
    (),
    DrawingAreaErrorKind<<PistonBackend<'a, 'b> as plotters::prelude::DrawingBackend>::ErrorType>,
> {
    let mut cc = ChartBuilder::on(&root)
        .margin(10)
        .caption("Spikiness", ("sans-serif", 20))
        .x_label_area_size(20)
        .y_label_area_size(25)
        .build_cartesian_2d(0..N_DATA_POINTS as u32, 0f32..0.5f32)?;

    cc.configure_mesh().draw()?;

    //Total
    cc.draw_series(LineSeries::new(
        (0..)
            .zip(data.spikiness.iter())
            .map(|(a, b)| (a, *b)),
        &Palette99::pick(0),
    ))?
    .label("Total")
    .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(0)));

    cc.configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}

pub fn create_window(monitor_data: Arc<Mutex<MonitorData>>) {
    let mut window: PistonWindow = WindowSettings::new("Real Time CPU Usage", [800, 500])
        .samples(4)
        .build()
        .unwrap();
    window.set_max_fps(FPS as u64);
    while let Some(_) = draw_piston_window(&mut window, |b| {
        let root = b.into_drawing_area();
        root.fill(&WHITE)?;
        let root = root.titled("nf_monitor", ("sans-serif", 25))?;
        let tiles = root.split_evenly((3, 1));
        let upper = tiles.get(0).unwrap();
        let middle = tiles.get(1).unwrap();
        let lower = tiles.get(2).unwrap();

        // lock data once
        let data = monitor_data.lock().unwrap();

        create_intensity_plot(&upper, &data)?;
        create_onset_plot(&middle, &data)?;
        create_spikiness_plot(&lower, &data)?;

        Ok(())
    }) {}
}
