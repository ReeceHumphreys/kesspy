use crate::satellite::Satellite;
use crate::SatKind;
use ndarray::*;
use ndarray_linalg::*;

pub trait FragmentationEvent {
    fn fragment_count(&self, min_characteristic_len: f32) -> f32;
    // Where the fragmentation event occured
    fn location(&self) -> Array1<f32>;
    fn max_characteristic_length(&self) -> f32;
    fn power_law_exponent(&self) -> f32;
    fn kind(&self) -> &SatKind;
    fn delta_velocity_offset(&self) -> [f32; 2];
}

pub struct CollisionEvent {
    pub max_characteristic_length: f32,
    pub sat_kind: SatKind,
    pub input_mass: f32,
    pub satellites: Array1<Satellite>,
}

impl CollisionEvent {
    pub fn new(satellites: &[Satellite]) -> CollisionEvent {
        let mut satellite_1 = satellites[0].clone();
        let mut satellite_2 = satellites[1].clone();
        let max_characteristic_length = satellite_1
            .characteristic_length
            .max(satellite_2.characteristic_length);
        let mut sat_kind = SatKind::SOC;
        if satellite_1.sat_kind == SatKind::RB || satellite_2.sat_kind == SatKind::RB {
            sat_kind = SatKind::RB;
        }

        let input_mass = satellite_1.mass + satellite_2.mass;

        // Fix satellite ordering so that first element is the larger satellite
        if satellite_2.characteristic_length > satellite_1.characteristic_length {
            std::mem::swap(&mut satellite_1, &mut satellite_2)
        }
        let satellites = array![satellite_1, satellite_2];

        CollisionEvent {
            max_characteristic_length,
            sat_kind,
            input_mass,
            satellites,
        }
    }

    /// Returns a the relative kinetic energy of the collision divided by the mass of the target. [J/g]
    ///
    /// # Arguments
    ///
    /// * `m_proj` - The mass of the projectile. [kg]
    /// * `m_targ` - The mass of the target. [g]
    /// * `v_impact` - The impact velocity of the collision. [m/s]
    fn rel_ke(&self, m_proj: f32, m_targ: f32, v_impact: f32) -> f32 {
        let ke = 0.5 * m_proj * v_impact.powi(2);
        ke / m_targ
    }

    /// Returns true if a collision is catastrophic and false if the collision is non-catastrophic.
    ///
    /// # Arguments
    ///
    /// * `m_proj` - The mass of the projectile. [kg]
    /// * `m_targ` - The mass of the target. [kg]
    /// * `v_impact` - The impact velocity of the collision. [km/s]
    fn is_catastrophic(&self, m_proj: f32, m_targ: f32, v_impact: f32) -> bool {
        let rel_ke = self.rel_ke(m_proj, m_targ * 1e3, v_impact * 1e3); // Need to convert km/s to m/s and kg to g
        let catastrophic_threshold = 40_f32; // [J/g]
        rel_ke > catastrophic_threshold
    }
}

impl FragmentationEvent for CollisionEvent {
    /// Returns the power law distribution for the number of fragments in a collision.
    ///
    /// # Arguments
    ///
    /// * `m_proj` - The mass of the projectile. [kg]
    /// * `m_targ` - The mass of the target. [kg]
    /// * `v_impact` - The impact velocity of the collision. [km/s]
    /// * `characteristic_len` - Then characteristic length. [m]
    fn fragment_count(&self, min_characteristic_len: f32) -> f32 {
        let satellite_1 = &self.satellites[0];
        let satellite_2 = &self.satellites[1];

        // Determine impact velocity
        let v_impact = (&satellite_1.velocity - &satellite_2.velocity).norm();

        // Target is the larger satellite
        let m_targ = satellite_1.mass;
        let m_proj = satellite_2.mass;
        let m = match self.is_catastrophic(m_proj, m_targ, v_impact) {
            true => m_proj + m_targ,
            false => m_proj * v_impact, // TODO: Have seen conflicting definitions of this. Need to find  most correct one.
        };
        0.1 * m.powf(0.75) * min_characteristic_len.powf(-1.71)
    }

    fn location(&self) -> Array1<f32> {
        self.satellites[0].position.to_owned()
    }

    fn max_characteristic_length(&self) -> f32 {
        self.max_characteristic_length
    }

    fn power_law_exponent(&self) -> f32 {
        -2.71
    }

    fn kind(&self) -> &SatKind {
        &self.sat_kind
    }

    fn delta_velocity_offset(&self) -> [f32; 2] {
        [0.9, 2.9]
    }
}

#[derive(Debug, Clone)]
pub struct ExplosionEvent {
    pub max_characteristic_length: f32,
    pub sat_type: SatKind,
    pub input_mass: f32,
    pub satellites: Array1<Satellite>,
}

impl ExplosionEvent {
    pub fn new(satellite: Satellite) -> ExplosionEvent {
        let max_characteristic_length = satellite.characteristic_length;
        let input_mass = satellite.mass;
        let sat_type = satellite.sat_kind.clone();

        ExplosionEvent {
            max_characteristic_length,
            input_mass,
            sat_type,
            satellites: array![satellite],
        }
    }
}

impl FragmentationEvent for ExplosionEvent {
    /// Returns the number of fragments in an explosion.
    ///
    /// # Arguments
    /// * `characteristic_len` - Then characteristic length. [m]
    fn fragment_count(&self, min_characteristic_len: f32) -> f32 {
        let s = 1.0;
        6_f32 * s * min_characteristic_len.powf(-1.6)
    }

    fn location(&self) -> Array1<f32> {
        self.satellites.get(0).unwrap().position.to_owned()
    }

    fn max_characteristic_length(&self) -> f32 {
        self.max_characteristic_length
    }

    fn power_law_exponent(&self) -> f32 {
        -2.6
    }

    fn kind(&self) -> &SatKind {
        &self.sat_type
    }

    fn delta_velocity_offset(&self) -> [f32; 2] {
        [0.2, 1.85]
    }
}
