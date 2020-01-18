
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;


use pyo3::{Python, PyResult};
use pyo3::types::{PyModule, PyDict};
use mpi::topology::Communicator;
use pyo3::wrap_pyfunction;

//use crate::neworder;

fn append_model_paths(paths: &[String]) {

  let pypath = match std::env::var("PYTHONPATH") {
    Ok(val) => val + ":",
    Err(_) => "".to_string()
  } + &paths.join(":");

  std::env::set_var("PYTHONPATH", pypath);
}

fn main() -> Result<(), ()> {

  let args = std::env::args().collect::<Vec<String>>();

  if args.len() < 2 {
    println!("usage: neworder <model-path> [<extra-path>...]");
    std::process::exit(1)
  }

  append_model_paths(&args[1..]);

  let gil = Python::acquire_gil();
  run(gil.python()).map_err(|e| {
    eprintln!("error! :{:?}", e);
    // we can't display python error type via ::std::fmt::Display
    // so print error here manually
    e.print_and_set_sys_last_vars(gil.python());
  })
}


// fn main() -> Result<(), ()> {
//   let gil = Python::acquire_gil();
//   let py = gil.python();
//   main_(py).map_err(|e| {
//       // We can't display python error type via ::std::fmt::Display,
//       // so print error here manually.
//       e.print_and_set_sys_last_vars(py);
//   })
// }

// fn main_(py: Python) -> PyResult<()> {
//   let sys = py.import("sys")?;
//   let version: String = sys.get("version")?.extract()?;
//   let locals = [("os", py.import("os")?)].into_py_dict(py);
//   let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
//   let user: String = py.eval(code, None, Some(&locals))?.extract()?;
//   println!("Hello {}, I'm Python {}", user, version);
//   Ok(())
// }

fn run<'py>(py: Python<'py>) -> PyResult<()> {

  let universe = mpi::initialize().unwrap();
  let world = universe.world();
  let size = world.size();
  let rank = world.rank();

  println!("MPI {}/{}", rank, size);
  
  println!("PYTHONPATH={}", std::env::var("PYTHONPATH").unwrap());

  init_embedded_module(py)?;

  let test = py.import("testmodule")?;
  // println!("{}", py.eval("dir(testmodule)", None, None)?);
  println!("{}", test.call0("func")?);

  // let locals = [("testmodule", py.import("testmodule")?)].into_py_dict(py);
  // py.eval("print(dir(testmodule))", None, Some(&locals))?;
  // let res = py.eval("testmodule.func()", None, Some(&locals))?;
  // println!("{}", res);

  //let neworder = wrap_pymodule!(neworder)(py);

  //let locals = [("testmodule", py.import("testmodule")?), ("neworder", py.import("neworder")?)].into_py_dict(py);
  // let res = py.eval("neworder.name()", None, Some(&locals))?;
  // println!("{}", res);

  Ok(())
}


#[pyfunction]
fn foo(i: i32) -> i32 {
  i + 5
}

//#[pymodule]
fn init_embedded_module(py: Python) -> PyResult<()> {
  let no = PyModule::new(py, "neworder")?;
  add_module(py, no);
  no.add("x", 42)?;

  no.add_wrapped(wrap_pyfunction!(foo))?;
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