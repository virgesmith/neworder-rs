
use pyo3::prelude::*;

use pyo3::{Python, PyResult};
//use pyo3::types::{PyAny, PyString};
use pyo3::wrap_pyfunction;

use pyo3::types::{PyModule, PyDict};

use crate::environment;


#[pyfunction]
fn name() -> &'static str {
  "neworder"
}

#[pyfunction]
#[name="log"]
pub fn log_py(py: Python, x: PyObject) -> PyResult<()> {
  let a = x.as_ref(py);
  log_impl("py", environment::mpi().0, environment::mpi().1, &a.str()?.to_string()?);
  Ok(())
}

pub fn log(msg: &str) {
  log_impl("no.rs", environment::mpi().0, environment::mpi().1, msg);
}

fn log_impl(ctx: &'static str, rank: i32, size: i32, msg: &str) {
  println!("[{} {}/{}] {}", ctx, rank, size, msg);
}


pub fn init_embedded(py: Python) -> PyResult<()> {
  let no = PyModule::new(py, "neworder")?;
  add_module(py, no);
  // no.add("x", 42)?;

  no.add_wrapped(wrap_pyfunction!(name))?;
  no.add_wrapped(wrap_pyfunction!(log_py))?;

  Ok(())
}

fn add_module(py: Python, module: &PyModule) {
  py.import("sys")
      .expect("failed to import python sys module")
      .dict()
      .get_item("modules")
      .expect("failed to get python modules dictionary")
      .downcast_mut::<PyDict>()
      .expect("failed to turn sys.modules into a PyDict")
      .set_item(module.name().expect("module missing name"), module)
      .expect("failed to inject module");
}