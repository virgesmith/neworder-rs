
use pyo3::prelude::*;
use pyo3::{Python, PyResult};
//use pyo3::types::{PyAny, PyString};
use pyo3::wrap_pyfunction;
use pyo3::types::{PyModule, PyDict, PyList, PyTuple, PyString};

//use numpy::PyArray1;

//use numpy::array::get_array_module;
use mpi::topology::Rank;

use crate::env;
use crate::timeline::{Timeline, isnever, /*array_isnever,*/ NEVER, DISTANT_PAST, FAR_FUTURE};
use crate::montecarlo::MonteCarlo;

#[pyfunction]
pub fn name() -> &'static str {
  "neworder.rs"
}

#[pyfunction]
pub fn version() -> &'static str {
  "0.0.1"
}

#[pyfunction]
fn shell(py: Python) -> PyResult<()> {

  if env::size() != 1 {
    log("WARNING: shell disabled in parallel mode, ignoring");
  } else {
    //let args = PyTuple::new(py, 0);
    let kwargs = PyDict::new(py);
    kwargs.set_item("banner", PyString::new(py, "[starting neworder debug shell]"))?;
    kwargs.set_item("exitmsg", PyString::new(py, "[exiting neworder debug shell]")).unwrap();
  
    let module = py.import("code").unwrap();
    let f = &module.get("interact").unwrap(); //.to_object(py);
    &f.call(PyTuple::empty(py), Some(kwargs)).unwrap();    
  //   .get_attr("interact")?.call;
  // /* py::object interpreter = */py::module::import("code").attr("interact")(*py::tuple(), **kwargs);
  }
  Ok(())
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

fn log_impl(ctx: &'static str, rank: Rank, size: Rank, msg: &str) {
  println!("[{} {}/{}] {}", ctx, rank, size, msg);
}

#[pyfunction]
pub fn rank() -> Rank {
  env::rank()
}

#[pyfunction]
pub fn size() -> Rank {
  env::size()
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

#[pyfunction]
#[name="sync"]
fn sync_py() {
  env::sync()
}

#[pyfunction]
#[name="gather"]
fn gather_py(py: Python, x: f64, rank: Rank) -> PyObject {
  match env::gather_into(rank, &x) {
    //Some(a) => PyArray1::<f64>::from_vec(py, a).to_owned(),
    Some(a) => {
      log(&format!("{:?}",a));
      PyList::new(py, a).to_object(py)
    },
    None => py.None()
  }
}

// #[pyfunction]
// #[name="gather"]
// fn gather_py(py: Python, x: f64, rank: Rank) -> Vec::<f64> {
//   match env::gather_into(rank, &x) {
//     Some(a) => a,
//     None => Vec::new()
//   }
// }


// #[pyfunction]
// #[name="isnever_a"]
// fn isnever_a_py(a: PyArray1<f64>) -> Py<PyArray1<bool>> {

//   // where to get py from...TODO does this suffice?
//   array_isnever(Python::acquire_gil().python(), &a)
// }


pub fn init_embedded(py: Python) -> PyResult<&PyModule> {
  let no = PyModule::new(py, "neworder")?;
  add_module(py, no);
  no.add_wrapped(wrap_pyfunction!(rank))?;
  no.add_wrapped(wrap_pyfunction!(size))?;
  // use the module to store global variables when not easy on the rust side
  // seeding settings INDEP/SEED are added on initialisation

  no.add_wrapped(wrap_pyfunction!(name))?;
  no.add_wrapped(wrap_pyfunction!(version))?;

  no.add_wrapped(wrap_pyfunction!(shell))?;
  
  no.add_wrapped(wrap_pyfunction!(log_py))?;

  // time-related
  no.add_wrapped(wrap_pyfunction!(never))?;
  no.add_wrapped(wrap_pyfunction!(distant_past))?;
  no.add_wrapped(wrap_pyfunction!(far_future))?;
  no.add_wrapped(wrap_pyfunction!(isnever_py))?;
  no.add_class::<Timeline>()?;

  // now add numpy
  //let _np = get_array_module(py)?;

  // MC
  no.add_class::<MonteCarlo>()?;

  // MPI-related
  no.add_wrapped(wrap_pyfunction!(sync_py))?;
  no.add_wrapped(wrap_pyfunction!(gather_py))?;

  Ok(no)
}

pub fn init_mc<'py>(py: Python, indep: bool, no: &'py PyModule) -> &'py mut MonteCarlo /*PyResult<PyObject>*/ { 
  // init the MC engine
  let mc = PyRefMut::new(py, MonteCarlo::new(env::rank(), env::size(), indep)).unwrap();
  // expose it to python
  no.add("mc", mc).unwrap();
  no.get("mc").unwrap().extract().unwrap()
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