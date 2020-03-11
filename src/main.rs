#[macro_use]
extern crate lazy_static;

use pyo3::prelude::*;
use pyo3::{Python, PyResult}; //, AsPyPointer};
use pyo3::types::*; 

use argparse::{ArgumentParser, StoreFalse, Collect};

mod neworder;
mod env;
mod timeline;
mod python;
mod test;
mod montecarlo;

use std::time::{/*Duration, */Instant};

use python::{Runtime, CommandType, CommandDict, CommandList};
use timeline::Timeline;

fn append_model_paths(paths: &[String]) {

  let pypath = match std::env::var("PYTHONPATH") {
    Ok(val) => val + ":",
    Err(_) => "".to_string()
  } + &paths.join(":");

  std::env::set_var("PYTHONPATH", pypath);
}


fn main() -> Result<(), ()> {

  let mut independent = true;
  let mut paths = Vec::new();

  match (std::env::var("VIRTUAL_ENV"), std::env::var("CONDA_DEFAULT_ENV")) {
    (Err(_), Err(_)) | (Ok(_), Ok(_)) => panic!("neworder needs to run in either a virtualenv or a conda env"),
    _ => ()
  }

  

  // this block limits scope of (mutable) borrows by ap.refer() method
  {
    let mut ap = ArgumentParser::new();
    ap.set_description("neworder embedded python microsimulation framework.");
    ap.refer(&mut independent)
        .add_option(&["-c", "--correlated"], StoreFalse, "Use same RNG stream for every process");
    ap.refer(&mut paths)
        .add_argument("path", Collect, "python path");
    ap.parse_args_or_exit();
  }

  append_model_paths(&paths);

  let gil = Python::acquire_gil();
  run(gil.python(), independent).map_err(|e| {
    eprintln!("error! :{:?}", e);
    // we can't display python error type via ::std::fmt::Display
    // so print error here manually
    e.print_and_set_sys_last_vars(gil.python());
  })
}


fn run<'py>(py: Python<'py>, independent: bool) -> PyResult<()> {

  let start_time = Instant::now();

  let no = neworder::init_embedded(py)?;

  // init the MC engine
  neworder::init_mc(py, independent, no);
  let mc = no.get("mc")?.to_object(py);
  let indep = mc.getattr(py, "indep")?.call0(py)?.extract::<bool>(py)?;
  let seed = mc.getattr(py, "seed")?.call0(py)?.extract::<u32>(py)?;

  let sys = py.import("sys")?;
  // required otherwise matplotlib fails to initialise
  let argv = PyList::new(py, &["neworder"]);
  sys.add("argv", argv)?;

  let pyinfo = sys.get("version")?.to_string().replace("\n", "");

  neworder::log(&format!("{} initialised: mc=(indep:{} seed:{}) python={}", 
    neworder::name(), indep, seed, &pyinfo)); 
  neworder::log(&format!("PYTHONPATH={}", std::env::var("PYTHONPATH").unwrap()));
  
  let config = py.import("config").expect("module config was not imported successfully");
  //neworder::log(&format!("{}", py.eval("dir(testmodule)", None, None)?));
  //neworder::log(&test.call0("func")?.str()?.to_string()?);
  //neworder::log(&format!("{}", config.call0("func")?));

  let globals = None;
  let locals = PyDict::new(py); 

  // pull everything defined in config into the root namespace
  // this should include neworder
  for (k, v) in config.dict().iter() {
    locals.set_item(k, v)?;
  }
  // TODO either import confoig module, or
  // loop over items in config and import them along the lins of:
  //locals.set_item("nsims", config.dict().get_item("nsims"))?;
  // initialisations: evaluated immediately
  let initialisations: &PyDict = no.get("initialisations")?.downcast_ref()?;
  for (k, v) in initialisations.iter() {
    neworder::log(&format!("initialisation:{}", k));
    let d: &PyDict = v.downcast_ref()?;
    let modulename = &d.get_item("module").unwrap().extract::<String>().expect("module not found");
    let classname = &d.get_item("class_").unwrap().extract::<String>().expect("class not found");
    let args = d.get_item("args").unwrap().downcast_ref::<PyTuple>().expect("args not found (must be tuple e.g. (x,))");
    let kwargs = match d.get_item("kwargs") {
      Some(d) => Some(d.downcast_ref::<PyDict>().expect("kwargs not found")),
      None => None
    };

    // module is a PyModule
    let module = py.import(&modulename)?;

    // Get the class (a &PyObject)
    let class = &module.get(classname)?.to_object(py);
    // Call the ctor, (result is a &PyObject)
    let object = &class.call(py, args, kwargs).expect("initialisation error");

    // add to locals
    locals.set_item(k, object)?;

    // // Get the method
    // let method = object.getattr(py, "get_name")?;
    // // Call it
    // let res = method.call0(py)?.extract::<String>(py)?; //.as_ref();
    // // Display result
    // neworder::log(&format!("get_name()={}",res));

    // // Call the __call__/operator() method
    // let res = object.call(py, (), None)?.extract::<String>(py)?;
    // neworder::log(&format!("result={}", res )); 

  }

  // get the python rumtime
  let runtime = Runtime::new(py, globals, Some(locals));
  //runtime.run(&("neworder.log(locals())", CommandType::Exec))?;

  // modifiers: (optional) list of exec, one per process
  let modifiers = match no.get("modifiers") {
    Ok(o) => {
      let list = o.downcast_ref::<PyList>()?;
      // TODO something more functional?
      let mut cbs = CommandList::new();
      for item in list {
        let code = item.extract::<&str>()?; 
        cbs.push((&code, CommandType::Exec));
      }
      assert!(cbs.len() == env::size() as usize || cbs.len() == 0, "modifier array must either empty or have an entry for each process");
      cbs
    },
    Err(_) => CommandList::new()
  };

  // transitions: dict of exec
  let transitions: &PyDict = no.get("transitions").expect("transitions").downcast_ref()?;
  let mut transition_callbacks = CommandDict::new();
  for (k, v) in transitions {
    let name = k.extract::<&str>().expect("transition key");
    let code = v.extract::<&str>().expect("transition code");
    transition_callbacks.insert(name, (code, CommandType::Exec));   
  }

  // checks: (optional) dict of eval
  let checks = match no.get("checks") {
    Ok(o) => {
      let dict = o.downcast_ref::<PyDict>()?;
      let mut cbs = CommandDict::new();
      for (k, v) in dict {
        let name = k.extract::<&str>()?;
        let code = v.extract::<&str>()?;
        cbs.insert(name, (code, CommandType::Eval));            
      }
      cbs
    },
    Err(_) => CommandDict::new()
  };

  // checckpoints: dict of exec
  let checkpoints: &PyDict = no.get("checkpoints")?.downcast_ref()?;
  let mut checkpoint_callbacks = CommandDict::new();
  for (k, v) in checkpoints {
    let name = k.extract::<&str>()?;
    let code = v.extract::<&str>()?; //downcast_ref::<PyString>()?.to_string()?.to_string();
    checkpoint_callbacks.insert(name, (code, CommandType::Exec));   
  }
  
  if modifiers.len() > 0 {
    neworder::log(&format!("applying modifier: {}", modifiers[env::rank() as usize].0));
    runtime.run(&modifiers[env::rank() as usize])?;
  }

  // TODO how to get ptr to python impl as rust type?
  //let mut timeline: Timeline = Py::<Timeline>::from_borrowed_ptr(no.get("timeline")?.as_ptr()).as_ref(py).into(); //.get();
  let timeline: &mut Timeline = no.get("timeline")?.extract()?;
  //let pytimeline = no.get("timeline")?.to_object(py);
  loop {
    //pytimeline.getattr(py, "next")?.call0(py)?;
    timeline.next();

    // let i = pytimeline.getattr(py, "index")?.call0(py)?.extract::<u32>(py)?;
    // let t = pytimeline.getattr(py, "time")?.call0(py)?.extract::<f64>(py)?;
    let i = timeline.index();
    let t = timeline.time();

    // implement transitions
    for (k, v) in &transition_callbacks {
      neworder::log(&format!("t={}({}) transition {}", t, i, k));
      runtime.run(v).expect("transition failed");
    }

    // 
    for (k, v) in &checks {
      neworder::log(&format!("t={}({}) check {}", t, i, k));
      match runtime.run(v)?.extract::<bool>(py)? {
        true => (),
        false => panic!("check failed")
      }
    }

    //if pytimeline.getattr(py, "at_checkpoint")?.call0(py)?.extract::<bool>(py)? {
    if timeline.at_checkpoint() {
      for (k, v) in &checkpoint_callbacks {
        neworder::log(&format!("t={}({}) checkpoint {}", t, i, k));
        runtime.run(v)?;  
      }
    }

    //if pytimeline.getattr(py, "at_end")?.call0(py)?.extract::<bool>(py)? { break; }
    if timeline.at_end() { break; }
  }
  neworder::log(&format!("Completed. exec time(s)={}", start_time.elapsed().as_secs_f64()));

  // wait for all processes to finish
  env::sync();
  
  Ok(())
}


