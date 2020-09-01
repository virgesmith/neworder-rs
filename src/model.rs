
use pyo3::prelude::*;

//mod timeline;
use crate::Timeline;

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
  timeline: Timeline
  // seeder
}


#[pymethods]
impl Model {

  #[new]
  fn __init__(timeline: Timeline /*, end: f64, checkpoints: Vec<u32>*/) -> Self {
    Model{ timeline: timeline }
  }

}