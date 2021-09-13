use crate::{handle, RAIN_DENSITY};
use anyhow::Result;
use std::io::{stdout, Write};

// Functions required to solve problem.
pub trait Landscape {
    type PointHeight: std::fmt::Debug + From<f64> + Clone;

    fn rain(
        &mut self,
        rain_distr: impl Fn(usize) -> Self::PointHeight,
        return_result: bool,
    ) -> Result<&[Self::PointHeight]>;

    fn rain_uniform(
        &mut self,
        cnt: Self::PointHeight,
        return_result: bool,
    ) -> Result<&[Self::PointHeight]> {
        self.rain(|_| cnt.clone(), return_result)
    }

    fn precision(&self) -> Self::PointHeight;
}

pub fn start(steps: usize, points: Vec<f64>) -> Result<()> {
    let mut stdout = stdout();
    let mut landscape = handle(points);
    for n in 1..=steps {
        match landscape.rain_uniform(RAIN_DENSITY.into(), true) {
            Ok(water_levels) => {
                stdout.write_all(
                    format!("{:?}", water_levels)
                        .trim_matches(&['[', ']'] as &[_])
                        .as_bytes(),
                )?;
                stdout.write(&[b'\n'])?;
            }
            Err(e) => {
                eprintln!("Error during {} st/th invocation of rain(): {}", n, e);
            }
        }
    }
    return Ok(());
}
