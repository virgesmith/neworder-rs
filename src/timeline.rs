
use pyo3::prelude::*;
use pyo3::{Py,Python};
use numpy::PyArray1 as nparray1d;
use numpy::PyArrayDyn as nparray;
use numpy::PyArray;


#[pyclass]
pub struct Timeline {
  // start time (0)
  start: f64,
  // finish time (corresponding to final checkpoint)
  end: f64,  
  // steps at which to perform extra steps (including end)
  checkpoints: Vec<u32>,
  // current timestep
  index: u32
}


// e.g. { 2020.0, 2050.0, [10,20,30] }
// gives 1 year timesteps with checkpoints at 2030.0, 2040.0 ending at 2050.0

// TODO can we implement iter()/enumerate() for this struct?



#[pymethods]
impl Timeline {

  // TODO call the ctor
  #[new]
  fn new_py(init: &PyRawObject, start: f64, end: f64, checkpoints: Vec<u32>) {
    assert!(start < end, "start time must be before end time");
    assert!(checkpoints.len() > 0);
    for i in 1..checkpoints.len() {
      assert!(checkpoints[i-1] < checkpoints[i], "checkpoints should be monotonically increasing");
    }

    init.init(Timeline {
      checkpoints: checkpoints,
      start: start,
      end: end,
      index: 0
    });
  }

  // TODO use static method
  // cannot have 2 news
  // #[new]
  // fn null_py(init: &PyRawObject) {
  //   init.init(Timeline {
  //     checkpoints: vec![1;1],
  //     start: 0.0,
  //     end: 0.0,
  //     index: 0
  //   });
  // }

  // curent timestep index
  pub fn index(&self) -> u32 {
    self.index
  }

  // current timestep time
  pub fn time(&self) -> f64 {
    self.start + (self.end - self.start) * (self.index as f64) / (self.checkpoints.last().unwrap().clone() as f64)
  }

  // timestep length
  pub fn dt(&self) -> f64 {
    (self.end - self.start) / (self.checkpoints.last().unwrap().clone() as f64)
  }

}

// methods not visible to python
impl Timeline {

  pub fn new(start: f64, end: f64, checkpoints: Vec<u32>) -> Self {
    assert!(start < end, "start time must be before end time");
    assert!(checkpoints.len() > 0);
    for i in 1..checkpoints.len() {
      assert!(checkpoints[i-1] < checkpoints[i], "checkpoints should be monotonically increasing");
    }

    Timeline {
      checkpoints: checkpoints,
      start: start,
      end: end,
      index: 0
    }
  }

  pub fn null() -> Self {
    Timeline {
      checkpoints: vec![1;1],
      start: 0.0,
      end: 0.0,
      index: 0
    }
  }

  // increment timestep (check for running off end)
  pub fn step(&mut self) {
    self.index += 1;
  }

  // is current index a checkpoint?
  pub fn at_checkpoint(&self) -> bool {
    match self.checkpoints.iter().find(|&&x| x == self.index) {
      Some(_) => true,
      None => false
    }
  }

  pub fn at_end(&self) -> bool {
    &self.index == self.checkpoints.last().unwrap()
  }

  pub fn reset(&mut self) {
    self.index = 0;
  }

  
  // TODO how to iterate over an nD array
  // pub fn array_isnever_nd(py: Python, a: &nparray<f64>) -> Py<nparray<bool>> {
  //   let r = a.as_slice().unwrap().iter().map(|&x| Timeline::isnever(x)).collect::<Vec<bool>>();
  //   //let res = nparray1d::new(py, a.dims(), false);
  //   let res = nparray::from_vec(py, r);
  //   res.to_owned()
  // }
}

impl Iterator for Timeline {
  type Item = (u32, f64);

  fn next(&mut self) -> Option<Self::Item> {
    match self.at_end() {
      false => { self.step(); Some((self.index(), self.time())) },
      true => None
    }
  }
}

// unequal to any other value 
pub const NEVER: f64 = std::f64::NAN;
// less than any other value
pub const DISTANT_PAST: f64 = -std::f64::INFINITY;
// greater than any other value
pub const FAR_FUTURE: f64 = std::f64::INFINITY;

// custom comparison (as nan comparison always false)
pub fn isnever(t: f64) -> bool {
  t.is_nan() 
}

//#[pyfunction]
pub fn array_isnever(py: Python, a: &nparray1d<f64>) -> Py<nparray1d<bool>> {
  let r = a.as_slice().unwrap().iter().map(|&x| isnever(x)).collect::<Vec<bool>>();
  //let res = nparray1d::new(py, a.dims(), false);
  let res = nparray1d::from_vec(py, r);
  res.to_owned()
}


