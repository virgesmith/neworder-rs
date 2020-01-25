#[macro_use]
extern crate lazy_static;

use pyo3::prelude::*;

use pyo3::{Python, PyResult};

use pyo3::types::*; 

mod neworder;
mod env;
mod timeline;
mod callback;
mod test;

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

  let pyinfo = py.import("sys")?.get("version")?.to_string().replace("\n", "");

  neworder::log(&format!("{} initialised: python={} indep={} seed={}", neworder::name(), &pyinfo, env::indep(), env::seed()));
  neworder::log(&format!("PYTHONPATH={}", std::env::var("PYTHONPATH").unwrap()));
  
  let config = py.import("config")?;
  //neworder::log(&format!("{}", py.eval("dir(testmodule)", None, None)?));
  //neworder::log(&test.call0("func")?.str()?.to_string()?);
  neworder::log(&format!("{}", config.call0("func")?));

  let initialisations: &PyDict = no.get("initialisations")?.downcast_ref()?;

  for (k, v) in initialisations.iter() {
    neworder::log(&format!("{}:", k));
    let d: &PyDict = v.downcast_ref()?;
    // for (k2, v2) in d {
    //   neworder::log(&format!("  {}: {}", k2, v2));
    // }
    let modulename = &d.get_item("module").unwrap().downcast_ref::<PyString>()?.to_string()?;
    let classname = &d.get_item("class_").unwrap().downcast_ref::<PyString>()?.to_string()?;
    let args = d.get_item("args").unwrap().downcast_ref::<PyTuple>()?;
    let kwargs = match d.get_item("kwargs") {
      Some(d) => Some(d.downcast_ref::<PyDict>()?),
      None => None
    };

    // module is a PyModule
    let module = py.import(&modulename)?;
  
    // Get the class (a &PyObject)
    let class = &module.get(classname)?.to_object(py);
    // Call the ctor, (result is a &PyObject)
    let object = &class.call(py, args, kwargs)?;

    // Get the method
    let method = object.getattr(py, "get_name")?;
    // Call it
    let res = method.call0(py)?; //.as_ref();
    // Display result
    neworder::log_py(py, res)?;

    // Call the __call__/operator() method
    let res = object.call(py, (), None)?; //.to_string()?;
    neworder::log_py(py, res)?; //&format!("result={:?}", res ));
  }  

  Ok(())
}


