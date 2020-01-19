#[macro_use]
extern crate lazy_static;

//use pyo3::prelude::*;

use pyo3::{Python, PyResult, PyObject};

use pyo3::types::{PyString, PyDict, PyList, PyTuple};

mod neworder;
mod env;

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

  let no = neworder::init_embedded(py)?;

  let pym = py.import("sys")?.get("version")?.to_string().replace("\n", "");

  neworder::log(&format!("{} initialised: python={} indep={} seed={}", neworder::name(), &pym, env::indep(), env::seed()));
  neworder::log(&format!("PYTHONPATH={}", std::env::var("PYTHONPATH").unwrap()));
  
  let config = py.import("config")?;
  //neworder::log(&format!("{}", py.eval("dir(testmodule)", None, None)?));
  //neworder::log(&test.call0("func")?.str()?.to_string()?);
  neworder::log(&format!("{}", config.call0("func")?));

  let initialisations: &PyDict = no.get("initialisations")?.downcast_ref()?;

  for (k, v) in initialisations.iter() {
    neworder::log(&format!("{}:", k));
    let d: &PyDict = v.downcast_ref()?;
    for (k2, v2) in d {
      neworder::log(&format!("  {}: {}", k2, v2));
    }
    let modulename = &d.get_item("module").unwrap().downcast_ref::<PyString>()?.to_string()?;
    let classname = &d.get_item("class_").unwrap().downcast_ref::<PyString>()?.to_string()?;
    let args: &PyList = d.get_item("parameters").unwrap().downcast_ref()?;
    // TODO TypeError
    //let args: &PyTuple = d.get_item("parameters").unwrap().downcast_ref()?;


    let module = py.import(&modulename)?;
    // TODO how to get a PyObject from a PyAny?
    // let class: &PyObject = module.get(classname)? .to_object(py);//.downcast_ref()?;
    // let object = class.call(py, args, None)?;
  }  

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


