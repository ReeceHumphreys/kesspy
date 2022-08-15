#[macro_use]
extern crate cpython;

use rand::distributions::{Distribution, Uniform};

// re export am_ratio from am_ratio.rs
mod am_ratio;
pub use self::am_ratio::*;

py_module_initializer!(libnsbm, initlibnsbm, PyInit_nsbm, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(
        py,
        "count_doubles",
        py_fn!(py, am_ratio(sat_kind: i32, characteristic_len: f32)),
    )?;
    Ok(())
});

pub enum SatKind {
    RB,
    SOC,
    SC,
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

fn mass(area: f32, am_ratio: f32) -> f32 {
    area / am_ratio
}

fn area(characteristic_len: f32) -> f32 {
    const BOUND: f32 = 0.00167;
    if characteristic_len < BOUND {
        const FACTOR: f32 = 0.540424;
        FACTOR * characteristic_len.powi(2)
    } else {
        const EXP: f32 = 2.0047077;
        const FACTOR: f32 = 0.556945;
        FACTOR * characteristic_len.powf(EXP)
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

fn power_law(x0: f32, x1: f32, n: i32, y: f32) -> f32 {
    let step = x1.powi(n + 1) - x0.powi(n + 1) * y + x0.powi(n + 1);
    step.powf((1 / (n + 1)) as f32)
}

fn characteristic_length_dist(
    min_characteristic_len: f32,
    max_characteristic_len: f32,
    exponent: i32,
) -> f32 {
    // Sampling a value from uniform distribution
    let uniform = Uniform::new_inclusive(0.0, 1.0);
    let mut rng = rand::thread_rng();
    let y = uniform.sample(&mut rng);

    // Sampling a value from power law distribution
    power_law(min_characteristic_len, max_characteristic_len, exponent, y)
}
