
use pyo3::prelude::*;

use pyo3::{Python, PyResult};
//use pyo3::types::{PyAny, PyString};
use pyo3::wrap_pyfunction;

use pyo3::types::{PyModule, PyDict};

use numpy::array::get_array_module;
//use numpy::PyArray1;


use crate::env;
use crate::timeline::{Timeline, isnever, /*array_isnever,*/ NEVER, DISTANT_PAST, FAR_FUTURE};


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
  NEVER
}

#[pyfunction]
fn distant_past() -> f64 {
  DISTANT_PAST
}

#[pyfunction]
fn far_future() -> f64 {
  FAR_FUTURE
}

// custom comparison (as nan comparison always false)
#[pyfunction]
#[name="isnever"]
fn isnever_py(t: f64) -> bool {
  isnever(t)
}

// #[pyfunction]
// #[name="isnever"]
// fn isnever_pyarray_py(a: PyArray1<f64>) -> Py<PyArray1<bool>> {

//   // where to get py from...TODO does this suffice?
//   array_isnever(Python::acquire_gil().python(), &a)
// }


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
  no.add_wrapped(wrap_pyfunction!(isnever_py))?;
  no.add_class::<Timeline>()?;

  // now add numpy
  let _np = get_array_module(py)?;
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