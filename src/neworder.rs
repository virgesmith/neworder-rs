
use pyo3::prelude::*;

use pyo3::{Python, PyResult};
//use pyo3::types::{PyAny, PyString};
use pyo3::wrap_pyfunction;

use pyo3::types::{PyModule, PyDict};

use crate::env;
use crate::timeline::Timeline;


#[pyfunction]
pub fn name() -> &'static str {
  "neworder.rs"
}

#[pyfunction]
#[name="log"]
pub fn log_py(py: Python, x: PyObject) -> PyResult<()> {
  let a = x.as_ref(py);
  log_impl("py", env::rank(), env::size(), &a.str()?.to_string()?);
  Ok(())
}

pub fn log(msg: &str) {
  log_impl("no", env::rank(), env::size(), msg);
}

fn log_impl(ctx: &'static str, rank: i32, size: i32, msg: &str) {
  println!("[{} {}/{}] {}", ctx, rank, size, msg);
}

#[pyfunction]
fn never() -> f64 {
  Timeline::NEVER
}

#[pyfunction]
fn distant_past() -> f64 {
  Timeline::DISTANT_PAST
}

#[pyfunction]
fn far_future() -> f64 {
  Timeline::FAR_FUTURE
}



pub fn init_embedded(py: Python) -> PyResult<&PyModule> {
  let no = PyModule::new(py, "neworder")?;
  add_module(py, no);
  // use the module to store global variables
  no.add("rank", env::rank())?;
  no.add("size", env::size())?;
  no.add("indep", env::indep())?;
  no.add("seed", env::seed())?;

  no.add_wrapped(wrap_pyfunction!(name))?;
  no.add_wrapped(wrap_pyfunction!(log_py))?;

  // time-related
  no.add_wrapped(wrap_pyfunction!(never))?;
  no.add_wrapped(wrap_pyfunction!(distant_past))?;
  no.add_wrapped(wrap_pyfunction!(far_future))?;

  Ok(no)
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