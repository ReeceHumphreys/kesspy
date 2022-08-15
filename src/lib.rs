mod am_ratio;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::{pyfunction, PyResult};
use rand::distributions::{Distribution, Uniform};

/// A Python module implemented in Rust.
#[pymodule]
fn nsbm(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(am_ratio::am_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(mass, m)?)?;
    m.add_function(wrap_pyfunction!(area, m)?)?;
    m.add_function(wrap_pyfunction!(characteristic_length_dist, m)?)?;
    Ok(())
}

#[derive(Debug)]
pub enum SatKind {
    RB = 0,
    SOC = 1,
    SC = 2,
}

impl<'source> FromPyObject<'source> for SatKind {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        let kind = ob.extract::<i32>()?;
        match kind {
            0 => Ok(SatKind::RB),
            1 => Ok(SatKind::SOC),
            2 => Ok(SatKind::SC),
            _ => Err(PyErr::new::<PyValueError, _>("Invalid SatKind")),
        }
    }
}

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

#[derive(Debug)]
enum FragmentationEventKind {
    Collision {
        m_proj: f32,
        m_targ: f32,
        v_impact: f32,
    },
    Explosion {
        m_satellite: f32,
        s: f32,
    },
}

/// Returns a the relative kinetic energy of the collision divided by the mass of the target. [J/g]
///
/// # Arguments
///
/// * `m_proj` - The mass of the projectile. [kg]
/// * `m_targ` - The mass of the target. [g]
/// * `v_impact` - The impact velocity of the collision. [m/s]
fn rel_ke(m_proj: f32, m_targ: f32, v_impact: f32) -> f32 {
    let ke = 0.5 * m_proj * v_impact.powi(2);
    ke / m_targ
}
#[pyfunction]
fn mass(area: f32, am_ratio: f32) -> PyResult<f32> {
    Ok(area / am_ratio)
}

#[pyfunction]
fn area(characteristic_len: f32) -> PyResult<f32> {
    const BOUND: f32 = 0.00167;
    if characteristic_len < BOUND {
        const FACTOR: f32 = 0.540424;
        Ok(FACTOR * characteristic_len.powi(2))
    } else {
        const EXP: f32 = 2.0047077;
        const FACTOR: f32 = 0.556945;
        Ok(FACTOR * characteristic_len.powf(EXP))
    }
}

/// Returns true if a collision is catastrophic and false if the collision is non-catastrophic.
///
/// # Arguments
///
/// * `m_proj` - The mass of the projectile. [kg]
/// * `m_targ` - The mass of the target. [kg]
/// * `v_impact` - The impact velocity of the collision. [km/s]
fn is_catastrophic(m_proj: f32, m_targ: f32, v_impact: f32) -> bool {
    let rel_ke = rel_ke(m_proj, m_targ * 1e3, v_impact * 1e3); // Need to convert km/s to m/s and kg to g
    let catastrophic_threshold = 40_f32; // [J/g]
    rel_ke > catastrophic_threshold
}

/// Returns the power law distribution for the number of fragments in a collision.
///
/// # Arguments
///
/// * `m_proj` - The mass of the projectile. [kg]
/// * `m_targ` - The mass of the target. [kg]
/// * `v_impact` - The impact velocity of the collision. [km/s]
/// * `characteristic_len` - Then characteristic length. [m]
fn num_frag_collision(m_proj: f32, m_targ: f32, v_impact: f32, characteristic_len: f32) -> f32 {
    let m = match is_catastrophic(m_proj, m_targ, v_impact) {
        true => m_proj + m_targ,
        false => m_proj * v_impact, // TODO: Have seen conflicting definitions of this. Need to find  most correct one.
    };
    0.1 * m.powf(0.75) * characteristic_len.powf(-1.71)
}

/// Returns the number of fragments in an explosion.
///
/// # Arguments
/// * `characteristic_len` - Then characteristic length. [m]
/// * `s` - Scaling factor []
fn num_frag_explosion(characteristic_len: f32, s: f32) -> f32 {
    6_f32 * s * characteristic_len.powf(-1.6)
}

fn num_fragments(kind: FragmentationEventKind, characteristic_len: f32) -> f32 {
    match kind {
        FragmentationEventKind::Collision {
            m_proj,
            m_targ,
            v_impact,
        } => num_frag_collision(m_proj, m_targ, v_impact, characteristic_len),
        FragmentationEventKind::Explosion { m_satellite: _, s } => {
            num_frag_explosion(characteristic_len, s)
        }
    }
}

fn power_law(x0: f32, x1: f32, n: f32, y: f32) -> f32 {
    let step = x1.powf(n + 1.) - x0.powf(n + 1.) * y + x0.powf(n + 1.);
    step.powf((1. / (n + 1.)) as f32)
}

#[pyfunction]
fn characteristic_length_dist(
    min_characteristic_len: f32,
    max_characteristic_len: f32,
    exponent: f32,
) -> PyResult<f32> {
    // Sampling a value from uniform distribution
    let uniform = Uniform::new_inclusive(0.0, 1.0);
    let mut rng = rand::thread_rng();
    let y = uniform.sample(&mut rng);

    // Sampling a value from power law distribution
    Ok(power_law(
        min_characteristic_len,
        max_characteristic_len,
        exponent,
        y,
    ))
}
