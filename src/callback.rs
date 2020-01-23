
use pyo3::prelude::*;

use pyo3::types::PyDict;

use pyo3::Python;


use std::collections::HashMap;


// TODO locals and globals should probably be refs with a lifetime
pub struct Callback<'a> {
  exec: bool,
  code: String,
  globals: Option<&'a PyDict>,
  locals: Option<&'a PyDict>
}

// pub type CallbackList = Vec<Callback>;
// pub type CallbackDict = HashMap<String, Callback>;

impl<'a> Callback<'a> {
  pub fn exec(code: String, globals: Option<&'a PyDict>, locals: Option<&'a PyDict>) -> Callback<'a> {
    Callback{ exec: true, code: code, globals: globals, locals: locals }
  }

  pub fn eval(code: String, globals: Option<&'a PyDict>, locals: Option<&'a PyDict>) -> Callback<'a> {
    Callback{ exec: false, code: code, globals: globals, locals: locals }
  }

  pub fn run(&self, py: Python) -> PyResult<PyObject> {
    match self.exec {
      true => match py.run(&self.code, self.globals, self.locals) {
        Ok(_) => Ok(py.None()),
        Err(e) => Err(e)
      },
      false => match py.eval(&self.code, self.globals, self.locals) {
        Ok(r) => Ok(r.to_object(py)),
        Err(e) => Err(e)
      }
    }
  } 
}


