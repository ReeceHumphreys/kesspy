use crate::SatKind;
use ndarray::prelude::*;

#[derive(Debug, Clone)]
pub struct Satellite {
    pub position: Array1<f32>,
    pub velocity: Array1<f32>,
    pub mass: f32,
    pub characteristic_length: f32,
    pub sat_kind: SatKind,
}
