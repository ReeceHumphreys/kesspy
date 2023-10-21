use kessler::{run, CollisionEvent, ExplosionEvent, SatKind, Satellite};
use numpy::{PyArray1, PyArray3, ToPyArray};
use pyo3::prelude::*;

// Wrappers for the structs provided by kessler
#[pyclass(name = "CollisionEvent")]
#[derive(Debug)]
pub struct CollisionEventPyWrapper(CollisionEvent);

#[pymethods]
impl CollisionEventPyWrapper {
    #[new]
    pub fn __new__(
        _py: Python,
        satellite_one: &SatellitePyWrapper,
        satellite_two: &SatellitePyWrapper,
        min_characteristic_length: f32,
    ) -> Self {
        let satellites = &[satellite_one.0.to_owned(), satellite_two.0.to_owned()];
        CollisionEventPyWrapper(CollisionEvent::new(satellites, min_characteristic_length))
    }
}

#[pyclass(name = "ExplosionEvent")]
#[derive(Debug)]
pub struct ExplosionEventPyWrapper(ExplosionEvent);

#[pymethods]
impl ExplosionEventPyWrapper {
    #[new]
    pub fn __new__(_py: Python, satellite: &SatellitePyWrapper, min_characteristic_length: f32) -> Self {
        let satellite = satellite.0.to_owned();
        ExplosionEventPyWrapper(ExplosionEvent::new(satellite, min_characteristic_length))
    }
}

#[pyclass(name = "Satellite")]
#[derive(Debug)]
pub struct SatellitePyWrapper(Satellite);

#[pymethods]
impl SatellitePyWrapper {
    #[new]
    pub fn __new__(
        _py: Python,
        position: &PyArray1<f32>,
        velocity: &PyArray1<f32>,
        mass: f32,
    ) -> Self {

        let position: [f32; 3] = {
            unsafe {
                let position_slice = position.as_slice().expect("Failed to get slice from position array");
                if position_slice.len() != 3 {
                    panic!("Position array must have a length of 3");
                }
                let mut arr = [0.0; 3];
                arr.copy_from_slice(position_slice);
                arr
            }
        };

        let velocity: [f32; 3] = {
            unsafe {
                let velocity_slice = velocity.as_slice().expect("Failed to get slice from velocity array");
                if velocity_slice.len() != 3 {
                    panic!("Velocity array must have a length of 3");
                }
                let mut arr = [0.0; 3];
                arr.copy_from_slice(velocity_slice);
                arr
            }
        };
        SatellitePyWrapper(
            Satellite::new(
                position,
                velocity,
                mass,
                SatKind::Rb //TODO
            )
        )
    }
}

// The name of the module must be the same as the rust package name
#[pymodule]
fn kesspy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn run_collision<'py>(py: Python<'py>, event: &CollisionEventPyWrapper) -> &'py PyArray3<f32> {
        let event = &event.0;
        let output = run(event);
        output.to_pyarray(py)
    }

    #[pyfn(m)]
    fn run_explosion<'py>(py: Python<'py>, event: &ExplosionEventPyWrapper) -> &'py PyArray3<f32> {
        let event = &event.0;
        let output = run(event);
        output.to_pyarray(py)
    }

    m.add_class::<ExplosionEventPyWrapper>()?;
    m.add_class::<CollisionEventPyWrapper>()?;
    m.add_class::<SatellitePyWrapper>()?;

    Ok(())
}
