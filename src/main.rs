#[macro_use]
extern crate lazy_static;

use pyo3::prelude::*;

use pyo3::{Python, PyResult};

mod module;
mod environment;

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

  // TODO MPI not initialised? 
  module::init_embedded(py)?;

  module::log(&format!("initialised indep={} seed={}", environment::indep(), environment::seed()));
  module::log(&format!("PYTHONPATH={}", std::env::var("PYTHONPATH").unwrap()));
  
  let test = py.import("testmodule")?;
  //module::log(&format!("{}", py.eval("dir(testmodule)", None, None)?));
  //module::log(&test.call0("func")?.str()?.to_string()?);
  module::log(&format!("{}", test.call0("func")?));

  //let args = pyo3::types::PyTuple::new(py, &[3.14;1]/*:impl IntoIterator<Item = T, IntoIter = U>*/);
  //println!("{}", test.call1("str", args)?);

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


