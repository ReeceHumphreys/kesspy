mod am_ratio;
pub mod event;
mod satellite;

pub use self::satellite::Satellite;
// use pyo3::exceptions::PyValueError;
// use pyo3::prelude::*;
// use pyo3::{pyfunction, PyResult};
use crate::event::FragmentationEvent;

use am_ratio::am_ratio;
use ndarray::{s, Array, Array1, Array2, Array3};
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;
use rand_distr::UnitSphere;

/// A Python module implemented in Rust.
// #[pymodule]
// fn nsbm(_py: Python, m: &PyModule) -> PyResult<()> {
//     Ok(())
// }

pub struct BreakupModel {}

impl BreakupModel {
    pub fn run(self, event: &impl FragmentationEvent) -> Array3<f32> {
        let characteristic_len_min = 0.01;
        let fragment_count = event.fragment_count(characteristic_len_min).floor() as usize;
        let location = event.location();
        let characteristic_len_max = event.max_characteristic_length();
        let power_law_exponent = event.power_law_exponent();

        // Assigning debris location
        let mut result: Array3<f32> = Array::zeros((fragment_count, 7, 3));
        result.slice_mut(s![.., 1, ..]).assign(&location);

        // Computing L_c for each debris
        let mut char_lens = Array1::zeros(fragment_count);
        char_lens.par_map_inplace(|x| {
            *x = characteristic_length_dist(
                characteristic_len_min,
                characteristic_len_max,
                power_law_exponent,
            );
        });
        result.slice_mut(s![.., 2, 0]).assign(&char_lens);

        // Computing A/M Ratio for each debris
        let am_ratios = am_ratio(event.kind(), &char_lens);
        result.slice_mut(s![.., 3, 0]).assign(&am_ratios);

        // Computing area for each debris
        let areas = area(&char_lens);
        result.slice_mut(s![.., 4, 0]).assign(&areas);

        // Computing mass for each debris
        result
            .slice_mut(s![.., 5, 0])
            .assign(&(&areas / &am_ratios));

        // TODO: Implement mass conservation

        // Computing debris velocity
        let velocities = velocity(event, &am_ratios);
        result.slice_mut(s![.., 6, ..]).assign(&velocities);

        result
    }
}

///

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SatKind {
    RB = 0,
    SOC = 1,
    SC = 2,
}

// impl<'source> FromPyObject<'source> for SatKind {
//     fn extract(ob: &'source PyAny) -> PyResult<Self> {
//         let kind = ob.extract::<i32>()?;
//         match kind {
//             0 => Ok(SatKind::RB),
//             1 => Ok(SatKind::SOC),
//             2 => Ok(SatKind::SC),
//             _ => Err(PyErr::new::<PyValueError, _>("Invalid SatKind")),
//         }
//     }
// }

impl From<i32> for SatKind {
    fn from(i: i32) -> Self {
        match i {
            0 => SatKind::RB,
            1 => SatKind::SOC,
            2 => SatKind::SC,
            _ => panic!("Invalid SatKind"),
        }
    }
}

fn area(characteristic_len: &Array1<f32>) -> Array1<f32> {
    let mut result = characteristic_len.clone();
    const BOUND: f32 = 0.00167;
    result.par_map_inplace(|x| {
        if *x < BOUND {
            const FACTOR: f32 = 0.540424;
            *x = FACTOR * x.powi(2)
        } else {
            const EXP: f32 = 2.0047077;
            const FACTOR: f32 = 0.556945;
            *x = FACTOR * x.powf(EXP)
        }
    });
    result
}

fn velocity(event: &dyn FragmentationEvent, am_ratios: &Array1<f32>) -> Array2<f32> {
    let mut chi = am_ratios.clone();
    chi.mapv_inplace(f32::log10);

    let delta_v_offset = event.delta_velocity_offset();
    let mean = delta_v_offset[0] * chi + delta_v_offset[1];
    let std_dev = 0.4;

    let mut rng = rand::thread_rng();
    let vel_mag: Array1<f32> =
        mean.mapv_into(|m| Normal::new(m, std_dev).unwrap().sample(&mut rng));
    let mut velocities = Array2::zeros((vel_mag.len(), 3));
    for (i, mut row) in velocities.axis_iter_mut(ndarray::Axis(0)).enumerate() {
        // Perform calculations and assign to `row`; this is a trivial example:
        let unit_vec: [f32; 3] = UnitSphere.sample(&mut rng);
        let mag = vel_mag[i];
        row[0] = mag * unit_vec[0];
        row[1] = mag * unit_vec[1];
        row[2] = mag * unit_vec[2]
    }
    velocities
}

fn power_law(x0: f32, x1: f32, n: f32, y: f32) -> f32 {
    let step = x1.powf(n + 1.) - x0.powf(n + 1.) * y + x0.powf(n + 1.);
    step.powf((1. / (n + 1.)) as f32)
}

fn characteristic_length_dist(
    min_characteristic_len: f32,
    max_characteristic_len: f32,
    exponent: f32,
) -> f32 {
    // Sampling a value from uniform distribution
    let uniform = Uniform::new_inclusive(0.0, 1.0);
    let mut rng = rand::thread_rng();
    let y = uniform.sample(&mut rng);

    // Sampling a value from power law distribution
    power_law(min_characteristic_len, max_characteristic_len, exponent, y)
}
