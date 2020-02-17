
use pyo3::prelude::*;

use pyo3::types::PyDict;

use pyo3::Python;


use std::collections::HashMap;


pub struct Runtime<'a> {
  py: Python<'a>,
  globals: Option<&'a PyDict>,
  locals: Option<&'a PyDict>
}

impl<'a> Runtime<'a> {
  pub fn new(py: Python<'a>, globals: Option<&'a PyDict>, locals: Option<&'a PyDict>) -> Runtime<'a> {
    Runtime{py: py, globals: globals, locals: locals }
  }

  pub fn run(&self, code: &(&'a str, CommandType)) -> PyResult<PyObject> {
    match &code.1 {
      CommandType::Exec => match self.py.run(code.0, self.globals, self.locals) {
        Ok(_) => Ok(self.py.None()),
        Err(e) => Err(e)
      },
      CommandType::Eval => match self.py.eval(code.0, self.globals, self.locals) {
        Ok(r) => Ok(r.to_object(self.py)),
        Err(e) => Err(e)
      }
    }
  } 
}

pub enum CommandType {
  Exec, // no return value
  Eval  // returns something
}

pub type Command<'a> = (&'a str, CommandType);

pub type CommandList<'a> = Vec<Command<'a>>;

pub type CommandDict<'a> = HashMap<&'a str, Command<'a>>;


