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
    ) -> Self {
        let satellites = &[satellite_one.0.to_owned(), satellite_two.0.to_owned()];
        CollisionEventPyWrapper(CollisionEvent::new(satellites))
    }
}

#[pyclass(name = "ExplosionEvent")]
#[derive(Debug)]
pub struct ExplosionEventPyWrapper(ExplosionEvent);

#[pymethods]
impl ExplosionEventPyWrapper {
    #[new]
    pub fn __new__(_py: Python, satellite: &SatellitePyWrapper) -> Self {
        let satellite = satellite.0.to_owned();
        ExplosionEventPyWrapper(ExplosionEvent::new(satellite))
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
        characteristic_length: f32,
    ) -> Self {
        let position = unsafe { position.as_array().to_owned() }; // Convert to ndarray type
        let velocity = unsafe { velocity.as_array().to_owned() }; // Convert to ndarray type
        SatellitePyWrapper(Satellite {
            position,
            velocity,
            mass,
            sat_kind: SatKind::Rb, // TODO
            characteristic_length,
        })
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
