
use pyo3::prelude::*;
// use pyo3::PyRef;
// //use pyo3::types::PyAny;
// use numpy::PyArray1;
// //use numpy::PyArrayDyn;
// //use numpy::PyArray;


#[pyclass]
#[derive(Clone)]
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


// e.g. Timeline{ 2020.0, 2050.0, [10,20,30] }
// gives 1 year timesteps with checkpoints at 2030.0, 2040.0 ending at 2050.0

#[pymethods]
impl Timeline {

  #[new]
  fn __init__(start: f64, end: f64, checkpoints: Vec<u32>) -> Self {
    Timeline::new(start, end, checkpoints)
  }

  #[staticmethod]
  pub fn null() -> Self {
    Timeline {
      checkpoints: vec![1;1],
      start: 0.0,
      end: 0.0,
      index: 0
    }
  }


  // Temporarily(?) expose an increment method to python
  fn next(&mut self) {
    self.index += 1;
  }

  // current timestep index
  pub fn index(&self) -> u32 {
    self.index
  }

  // current timestep time
  pub fn time(&self) -> f64 {
    self.start + (self.end - self.start) * (self.index as f64) / (*self.checkpoints.last().unwrap() as f64)
  }

  // timestep length
  pub fn dt(&self) -> f64 {
    (self.end - self.start) / (*self.checkpoints.last().unwrap() as f64)
  }

  // is current index a checkpoint?
  pub fn at_checkpoint(&self) -> bool {
    self.checkpoints.iter().any(|&x| x == self.index) //find(|&&x| x == self.index).is_some()
  }

  pub fn at_end(&self) -> bool {
    &self.index == self.checkpoints.last().unwrap()
  }

  pub fn nsteps(&self) -> u32 {
    *self.checkpoints.last().unwrap()
  }

  // #[staticmethod]
  // pub fn array_isnever(py: Python, a: &PyArray1<f64>) -> Py<PyArray1<bool>> {
  //   let r = a.as_slice().unwrap().iter().map(|&x| isnever(x)).collect::<Vec<bool>>();
  //   //let res = PyArray1::new(py, a.dims(), false);
  //   let res = PyArray1::from_vec(py, r);
  //   res.to_owned()
  // }


  // this doesnt work unless explicitly called e.g. timeline.__repr__()
  fn __repr__(&self) -> PyResult<String> {
    Ok(format!("<neworder.Timeline start={} end={}, checkpoints={:?} index={}>", self.start, self.end, self.checkpoints, self.index))
  }

  fn __str__(&self) -> PyResult<String> {
    self.__repr__()
  }

  // fn as_pyref(slf: PyRef<Self>, py: Python) -> PyResult<PyObject> {
  //   Ok(slf.to_object(py))
  // }

}

// methods not visible to python
impl Timeline {

  pub fn new(start: f64, end: f64, checkpoints: Vec<u32>) -> Self {
    assert!(start < end, "start time must be before end time");
    assert!(!checkpoints.is_empty());
    for i in 1..checkpoints.len() {
      assert!(checkpoints[i-1] < checkpoints[i], "checkpoints should be monotonically increasing");
    }

    Timeline {
      checkpoints,
      start,
      end,
      index: 0
    }
  }

  pub fn reset(&mut self) {
    self.index = 0;
  }


  // TODO how to iterate over an nD array
  // pub fn array_isnever_nd(py: Python, a: &nparray<f64>) -> Py<nparray<bool>> {
  //   let r = a.as_slice().unwrap().iter().map(|&x| Timeline::isnever(x)).collect::<Vec<bool>>();
  //   //let res = PyArray1<?>::new(py, a.dims(), false);
  //   let res = nparray::from_vec(py, r);
  //   res.to_owned()
  // }
}

impl Iterator for Timeline {
  type Item = (u32, f64);

  fn next(&mut self) -> Option<Self::Item> {
    match self.at_end() {
      false => { self.index += 1; Some((self.index(), self.time())) },
      true => None
    }
  }
}


// unequal to any other value
pub const NEVER: f64 = std::f64::NAN;
// less than any other value
pub const DISTANT_PAST: f64 = std::f64::NEG_INFINITY;
// greater than any other value
pub const FAR_FUTURE: f64 = std::f64::INFINITY;

// custom comparison (as nan comparison always false)
pub fn isnever(t: f64) -> bool {
  t.is_nan()
}

