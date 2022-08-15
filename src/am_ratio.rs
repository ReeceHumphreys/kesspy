use crate::SatKind;

use pyo3::{pyfunction, PyResult};
use rand::distributions::Distribution;
use rand_distr::Normal;

/// Returns the area of a fragment [m].
///
/// # Arguments
/// * `characteristic_len` - Then characteristic length. [m]
#[pyfunction]
pub fn am_ratio(sat_kind: SatKind, characteristic_len: f32) -> PyResult<f32> {
    let lambda_lc = characteristic_len.log10();
    if lambda_lc > 0.11 {
        // Bigger than 11 cm
        // Means and Std Devs. for normal distributions
        let mean_1 = mean_1(&sat_kind, lambda_lc);
        let mean_2 = mean_2(&sat_kind, lambda_lc);
        let std_dev_1 = sigma_1(&sat_kind, lambda_lc);
        let std_dev_2 = sigma_2(&sat_kind, lambda_lc);

        // Sample values from normal distributions
        let mut rng = rand::thread_rng();

        let normal_1 = Normal::new(mean_1, std_dev_1).unwrap();
        let normal_2 = Normal::new(mean_2, std_dev_2).unwrap();

        let n1 = normal_1.sample(&mut rng);
        let n2 = normal_2.sample(&mut rng);

        // Calculate AM Ratio
        Ok(10_f32.powf(alpha(&sat_kind, lambda_lc) * n1 + (1. - alpha(&sat_kind, lambda_lc)) * n2))
    } else if lambda_lc < 0.08 {
        // Smaller than 8 cm

        // Sample a random value from the normal distribution
        let mean = mean_soc(lambda_lc);
        let std_dev = sigma_soc(lambda_lc);
        let normal = Normal::new(mean, std_dev).unwrap(); // TODO: Handle error
        let mut rng = rand::thread_rng();
        let y = normal.sample(&mut rng);

        Ok(10_f32.powf(y))
    } else {
        // Case between 8 cm and 11 cm
        // Means and Std Devs. for normal distributions
        let mean_1 = mean_1(&sat_kind, lambda_lc);
        let mean_2 = mean_2(&sat_kind, lambda_lc);
        let std_dev_1 = sigma_1(&sat_kind, lambda_lc);
        let std_dev_2 = sigma_2(&sat_kind, lambda_lc);

        // Sample values from normal distributions
        let mut rng = rand::thread_rng();

        let normal_1 = Normal::new(mean_1, std_dev_1).unwrap();
        let normal_2 = Normal::new(mean_2, std_dev_2).unwrap();

        let n = Normal::new(mean_soc(lambda_lc), sigma_soc(lambda_lc))
            .unwrap()
            .sample(&mut rng);
        let n1 = normal_1.sample(&mut rng);
        let n2 = normal_2.sample(&mut rng);

        let y0 = 10_f32.powf(n);
        let y1 = 10_f32
            .powf(alpha(&sat_kind, lambda_lc) * n1 + (1.0 - alpha(&sat_kind, lambda_lc)) * n2);

        Ok(y0 + (characteristic_len - 0.08) * (y1 - y0) / (0.03))
    }
}

/* ------------------ Helpers ------------------ */
fn distribution_constant<F>(
    lambda_lc: f32,
    lower: f32,
    upper: f32,
    lower_return: f32,
    upper_return: f32,
    mid_function: F,
) -> f32
where
    F: Fn(f32) -> f32,
{
    if lambda_lc <= lower {
        lower_return
    } else if lambda_lc >= upper {
        upper_return
    } else {
        mid_function(lambda_lc)
    }
}

/* ------------------ Means and Std. Devs ------------------ */
fn alpha(sat_kind: &SatKind, lambda_lc: f32) -> f32 {
    match sat_kind {
        SatKind::RB => distribution_constant(lambda_lc, -1.4, 0.0, 1.0, 0.5, |lambda_lc| {
            1.0 - 0.3571 * (lambda_lc + 1.4)
        }),
        _ => distribution_constant(lambda_lc, -1.95, 0.55, 0.0, 1.0, |lambda_lc| {
            0.3 + 0.4 * (lambda_lc + 1.2)
        }),
    }
}

// Handles RB and SC case
fn mean_1(sat_kind: &SatKind, lambda_lc: f32) -> f32 {
    match sat_kind {
        SatKind::RB => distribution_constant(lambda_lc, -0.5, 0.0, -0.45, -0.9, |lambda_lc| {
            -0.45 - 0.9 * (lambda_lc + 0.5)
        }),
        _ => distribution_constant(lambda_lc, -1.1, 0.0, -0.6, -0.95, |lambda_lc| {
            -0.6 - std::f32::consts::FRAC_1_PI * (lambda_lc + 1.1)
        }),
    }
}

// Handles RB and SC case
fn sigma_1(sat_kind: &SatKind, lambda_lc: f32) -> f32 {
    match sat_kind {
        SatKind::RB => 0.55,
        _ => distribution_constant(lambda_lc, -1.3, -0.3, 0.1, 0.3, |lambda_lc| {
            0.1 + 0.2 * (lambda_lc + 1.3)
        }),
    }
}

// Handles RB and SC case
fn mean_2(sat_kind: &SatKind, lambda_lc: f32) -> f32 {
    match sat_kind {
        SatKind::RB => -0.9,
        _ => distribution_constant(lambda_lc, -0.7, -0.1, -1.2, -2.0, |lambda_lc| {
            -1.2 - 1.333 * (lambda_lc + 0.7)
        }),
    }
}

// Handles RB and SC case
fn sigma_2(sat_kind: &SatKind, lambda_lc: f32) -> f32 {
    match sat_kind {
        SatKind::RB => distribution_constant(lambda_lc, -0.5, -0.3, 0.5, 0.3, |lambda_lc| {
            0.5 - (lambda_lc + 0.5)
        }),
        _ => distribution_constant(lambda_lc, -0.5, -0.3, 0.5, 0.3, |lambda_lc| {
            0.5 - (lambda_lc + 0.5)
        }),
    }
}

fn mean_soc(lambda_lc: f32) -> f32 {
    distribution_constant(lambda_lc, -1.75, -1.25, -0.3, -1.0, |lambda_lc| {
        -0.3 - 1.4 * (lambda_lc + 1.75)
    })
}

fn sigma_soc(lambda_lc: f32) -> f32 {
    if lambda_lc <= -3.5 {
        0.2
    } else {
        0.2 + 0.1333 * (lambda_lc + 3.5)
    }
}
