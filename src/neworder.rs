
use pyo3::prelude::*;

use pyo3::{Python, PyResult};
//use pyo3::types::{PyAny, PyString};
use pyo3::wrap_pyfunction;

use pyo3::types::{PyModule, PyDict};

use numpy::array::get_array_module;

use crate::env;
use crate::timeline::{Timeline, isnever, NEVER, DISTANT_PAST, FAR_FUTURE};


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


// // TODO just use isnever?
// #[pyfunction]
// #[name="isnever"]
// fn isnever_py(t: f64) -> bool {
//   isnever(t)
// }

// #[pyfunction]
// #[name="isnever"]
// fn isnever_pyarray(a: nparray1d<f64>) -> Py<nparray1d<bool>> {
//   // where to get py from...
//   Timeline::array_isnever(py, a)
// }


#[pyclass]
pub struct TestClass {
  i: i32,
  x: f64
}

// #[pymethods]
// impl TestClass {
//   #[new]
//   fn new_py(init: &PyRawObject, i: i32, x: f64) { 
//     init.init(TestClass{i:i, x:x});
//   }

//   fn next(&mut self) { //-> &mut Self {
//     self. i += 1;
//     self. x += 0.01;
//     //&mut self
//   }

//   fn foo(&self) -> f64 {
//     self.x
//   }
// }

// // not exposed to py
// impl TestClass {
//   fn new(i: i32, x: f64) -> Self { 
//     TestClass{i:i, x:x}
//   }

//   fn bar(&self) -> i32 {
//     self.i
//   }
// }


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