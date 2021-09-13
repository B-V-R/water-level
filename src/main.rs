use crate::app::{start, Landscape};

// Amount of rain that falls onto one point (segment) in one step (1h).
const RAIN_DENSITY: f64 = 1.0;

mod app;
mod rain_landscapes;
mod util;

fn handle(points_heights: Vec<f64>) -> impl Landscape {
    rain_landscapes::Landscape::create(points_heights)
}

// Program main function.
fn main() {
    println!("Enter rain hours");
    let steps = util::read_input_rain_hours();

    println!("Enter landscape heights: ,ex: 1 2 3");
    let points = util::read_input();

    start(steps, points);
}
