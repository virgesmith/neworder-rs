
use pyo3::prelude::*;
//use pyo3::conversion::FromPyObject;
use pyo3::exceptions::NotImplementedError;

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

#[pyclass]
pub struct Model {
  timeline: Timeline,
  mc: MonteCarlo
}


#[pymethods]
impl Model {

  #[new]
  fn __init__(py: Python, timeline: Timeline, seeder: PyObject) -> Self {
    //let args = PyTuple:new(py, &vec![env::rank(); 1]);
    //let args = (.into_tuple(py);
    let seed: i64 = seeder.call1(py, (env::rank(),)).unwrap().extract(py).unwrap();
    Model{ timeline: timeline, mc: MonteCarlo::new(seed) }
  }

  // fn timeline(&self) -> &Timeline {
  //   &self.timeline
  // }

  // fn mc(&self) -> PyResult<&MonteCarlo> {
  //   Ok(&self.mc)
  // }

  // the trait `pyo3::callback::IntoPyCallbackOutput<_>` is not implemented for `std::result::Result<&montecarlo::MonteCarlo, pyo3::PyErr>`  

  fn modify(&self, _r: i32) -> PyResult<()> {
    Ok(())
  }

  fn step(&self) -> PyResult<()> {
    NotImplementedError::into("step() must be implemented in the model subclass")
  }

  fn check(&self) -> PyResult<bool> {
    neworder::log("no-op Model::check()");
    Ok(true)
  }

  fn checkpoint(&self) -> PyResult<()> {
    NotImplementedError::into("checkpoint() must be implemented in the model subclass")
  }

}