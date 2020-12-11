
use pyo3::prelude::*;
use crate::env;
use crate::montecarlo::MonteCarlo;
use crate::timeline;
use crate::timeline::Timeline;
use crate::model::Model;
//use pyo3::wrap_pymodule;

pub fn log(msg: &str) {
  log_impl("no", env::rank(), env::size(), msg);
}

fn log_impl(ctx: &'static str, rank: i32, size: i32, msg: &str) {
  println!("[{} {}/{}] {}", ctx, rank, size, msg);
}

#[pymodule]
fn neworder(py: Python, m: &PyModule) -> PyResult<()> {

  #[pyfn(m, "version")]
  fn version(_py: Python) -> PyResult<&str> {
    Ok(env::version())
  }

  // default arg=True
  #[pyfn(m, "verbose")]
  fn verbose_default(_py: Python, b: Option<bool>) -> PyResult<()> {
    match b {
      Some(b) => Ok(env::verbose(b)),
      None => Ok(env::verbose(true))
    }
  }

  // default arg=True
  #[pyfn(m, "checked")]
  fn checked(_py: Python, b: Option<bool>) -> PyResult<()> {
    match b {
      Some(b) => Ok(env::checked(b)),
      None => Ok(env::checked(true))
    }
  }

  #[pyfn(m, "halt")]
  fn halt(_py: Python) -> PyResult<()> {
    Ok(env::halt(true))
  }

  // TODO try to import mpi4py and check mpi env

  // mpi submodule
  let mpi = PyModule::new(py, "mpi")?;

  #[pyfn(mpi, "rank")]
  fn rank() -> PyResult<i32> {
    Ok(env::rank())
  }

  #[pyfn(mpi, "size")]
  fn size() -> PyResult<i32> {
    Ok(env::size())
  }
  m.add_submodule(mpi)?;

  // time submodule
  let time = PyModule::new(py, "time")?;

  #[pyfn(time, "distant_past")]
  fn distant_past() -> PyResult<f64> {
    Ok(timeline::DISTANT_PAST)
  }

  #[pyfn(time, "far_future")]
  fn far_future() -> PyResult<f64> {
    Ok(timeline::FAR_FUTURE)
  }

  #[pyfn(time, "never")]
  fn never() -> PyResult<f64> {
    Ok(timeline::NEVER)
  }

  #[pyfn(time, "isnever")]
  fn isnever(t: f64) -> PyResult<bool> {
    Ok(timeline::isnever(t))
  }

  m.add_submodule(time)?;

  // time submodule
  let stats = PyModule::new(py, "stats")?;
  // TODO: stats submodule
  m.add_submodule(stats)?;

  let df = PyModule::new(py, "df")?;
  // TODO: df submodule
  m.add_submodule(df)?;


  #[pyfn(m, "log")]
  pub fn log_py(py: Python, x: PyObject) -> PyResult<()> {
    let a = x.as_ref(py);
    log_impl("py", env::rank(), env::size(), &a.to_string());
    Ok(())
  }

  m.add_class::<Model>()?;
  m.add_class::<Timeline>()?;
  m.add_class::<MonteCarlo>()?;

  Ok(())
}
