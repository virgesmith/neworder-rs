use pyo3::prelude::*;

// This is for demonstrating how to return a value from __next__
#[pyclass]
pub struct PyClassTest {
    count: usize,
}

#[pymethods]
impl PyClassTest {
    #[new]
    pub fn new() -> Self {
        PyClassTest { count: 0 }
    }
}