#[macro_use]
extern crate lazy_static;

use pyo3::prelude::*;

use pyo3::{Python, PyResult, AsPyPointer};

use pyo3::types::*; 

mod neworder;
mod env;
mod timeline;
mod callback;
mod test;

use callback::{Callback, CallbackDict, CallbackList};
use timeline::Timeline;

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
  //neworder::log(&format!("{}", config.call0("func")?));

  let globals = None;
  let locals = None; // TODO import neworder

  // initialisations: evaluated immediately
  let initialisations: &PyDict = no.get("initialisations")?.downcast_ref()?;
  for (k, v) in initialisations.iter() {
    neworder::log(&format!("{}:", k));
    let d: &PyDict = v.downcast_ref()?;
    // for (k2, v2) in d {
    //   neworder::log(&format!("  {}: {}", k2, v2));
    // }
    let modulename = &d.get_item("module").unwrap().extract::<String>()?;
    let classname = &d.get_item("class_").unwrap().extract::<String>()?;
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
    let res = method.call0(py)?.extract::<String>(py)?; //.as_ref();
    // Display result
    neworder::log(&format!("get_name()={}",res));

    // Call the __call__/operator() method
    let res = object.call(py, (), None)?; //.to_string()?;
    neworder::log_py(py, res)?; //&format!("result={:?}", res )); 

  }

  // modifiers: (optional) list of exec, one per process
  let modifiers = match no.get("modifiers") {
    Ok(o) => {
      let list = o.downcast_ref::<PyList>()?;
      // TODO something more functional?
      let mut cbs = CallbackList::new();
      for item in list {
        let code = item.extract::<String>()?; 
        cbs.push(Callback::exec(code, globals, locals));
      }
      assert!(cbs.len() == env::size() as usize, "modifier array must have an entry for each process");
      cbs
    },
    Err(_) => CallbackList::new()
  };

  // transitions: dict of exec
  let transitions: &PyDict = no.get("transitions")?.downcast_ref()?;
  let mut transition_callbacks = CallbackDict::new();
  for (k, v) in transitions {
    let name = k.extract::<String>()?; //downcast_ref::<PyString>()?.to_string()?.to_string();
    let code = v.extract::<String>()?; //downcast_ref::<PyString>()?.to_string()?.to_string();
    transition_callbacks.insert(name, Callback::exec(code, globals, locals));   
  }

  // checks: (optional) dict of eval
  let checks = match no.get("checks") {
    Ok(o) => {
      let dict = o.downcast_ref::<PyDict>()?;
      let mut cbs = CallbackDict::new();
      for (k, v) in dict {
        let name = k.extract::<String>()?;
        let code = v.extract::<String>()?;
        cbs.insert(name, Callback::eval(code, globals, locals));            
      }
      cbs
    },
    Err(_) => CallbackDict::new()
  };

  // checckpoints: dict of exec
  let checkpoints: &PyDict = no.get("checkpoints")?.downcast_ref()?;
  let mut checkpoint_callbacks = CallbackDict::new();
  for (k, v) in checkpoints {
    let name = k.extract::<String>()?; //downcast_ref::<PyString>()?.to_string()?.to_string();
    let code = v.extract::<String>()?; //downcast_ref::<PyString>()?.to_string()?.to_string();
    checkpoint_callbacks.insert(name, Callback::exec(code, globals, locals));   
  }
  
  // // Apply any modifiers for this process
  // if (!modifierArray.empty())
  // {
  //   no::log("t=%%(%%) modifier: %%"_s % env.timeline().time() % env.timeline().index() % modifierArray[env.rank()].code());
  //   modifierArray[env.rank()]();
  // }
  if modifiers.len() > 0 {
    neworder::log(&format!("applying modifier to process {}: {}", env::rank(), modifiers[env::rank() as usize].code_()));
    modifiers[env::rank() as usize].run(py)?;
  }


  // TODO how to get ptr to python impl
  //let mut timeline: Timeline = Py::<Timeline>::from_borrowed_ptr(no.get("timeline")?.as_ptr()).as_ref(py).into(); //.get();
  //let timeline: Timeline = no.get("timeline")?.extract::<Timeline>()?;
  let pytimeline = no.get("timeline")?.to_object(py);
  loop {
    pytimeline.getattr(py, "next")?.call0(py)?;

    let i = pytimeline.getattr(py, "index")?.call0(py)?.extract::<u32>(py)?;
    let t = pytimeline.getattr(py, "time")?.call0(py)?.extract::<f64>(py)?;

    // implement transitions
    for (k, v) in &transition_callbacks {
      neworder::log(&format!("t={}({}) transition {}", t, i, k));
      v.run(py)?;
    }

    // 
    for (k, v) in &checks {
      neworder::log(&format!("t={}({}) check {}", t, i, k));
      match v.run(py)?.extract::<bool>(py)? {
        true => (),
        false => panic!("check failed")
      }
    }

    if pytimeline.getattr(py, "at_checkpoint")?.call0(py)?.extract::<bool>(py)? {
      for (k,v) in &checkpoint_callbacks {
        neworder::log(&format!("t={}({}) checkpoint {}", t, i, k));
        v.run(py)?;  
      }
    }

    if pytimeline.getattr(py, "at_end")?.call0(py)?.extract::<bool>(py)? { break; }
  }
  
  Ok(())
}


