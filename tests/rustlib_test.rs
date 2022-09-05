use ndarray::prelude::*;
use nsbm::event::ExplosionEvent;
use nsbm::{BreakupModel, SatKind, Satellite};

#[test]
fn my_test() {
    let my_sat = Satellite {
        position: Array1::from(vec![0., 1., 2.]),
        velocity: Array1::from(vec![0., 1., 2.]),
        mass: 1.0,
        characteristic_length: 0.1,
        sat_kind: SatKind::RB,
    };
    let model = BreakupModel {};
    let event = ExplosionEvent::new(my_sat);
    model.run(&event);
}
