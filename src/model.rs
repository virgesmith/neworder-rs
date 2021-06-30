

//use std::borrow::Borrow;
use pyo3::prelude::*;
//use pyo3::PyRef;
//use pyo3::conversion::FromPyObject;
use pyo3::exceptions::PyNotImplementedError;

//mod timeline;
use crate::env;
use crate::neworder;
use crate::timeline::Timeline;
use crate::montecarlo::MonteCarlo;
// Microsimulation (or ABM) model class
// py::class_<no::Model>(m, "Model")
// .def(py::init<no::Timeline&, const py::function&>())
// .def("timeline", &no::Model::timeline, py::return_value_policy::reference)
// .def("mc", &no::Model::mc, py::return_value_policy::reference)
// .def("modify", &no::Model::modify)
// .def("step", &no::Model::step)
// .def("check", &no::Model::check)
// .def("checkpoint", &no::Model::checkpoint);

#[pyclass(subclass)]
#[derive(Clone)]
pub struct Model {
  #[pyo3(get)]
  pub timeline: Timeline,
  #[pyo3(get)]
  pub mc: MonteCarlo
  //test: u32
}


#[pymethods]
impl Model {

  #[new]
  fn __init__(py: Python, timeline: Timeline, seeder: PyObject) -> Self {
    let seed: u32 = seeder.call1(py, (env::rank(),)).unwrap().extract(py).unwrap();
    Model{ timeline, mc: MonteCarlo::new(seed) }
  }

  fn modify(&self, _r: i32) -> PyResult<()> {
    neworder::log("no-op Model::modify()");
    Ok(())
  }

  fn step(&self) -> PyResult<()> {
    Err(PyNotImplementedError::new_err("step() must be implemented in the model subclass"))
  }

  fn check(&self) -> PyResult<bool> {
    neworder::log("no-op Model::check()");
    Ok(true)
  }

  fn checkpoint(&self) -> PyResult<()> {
    Err(PyNotImplementedError::new_err("checkpoint() must be implemented in the model subclass"))
  }

}


