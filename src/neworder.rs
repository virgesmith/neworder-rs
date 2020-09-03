
use pyo3::prelude::*;
use crate::env;
use crate::montecarlo::MonteCarlo;
use crate::timeline;
use crate::timeline::Timeline;
use crate::model::Model;
use pyo3::wrap_pymodule;

pub fn log(msg: &str) {
  log_impl("no", env::rank(), env::size(), msg);
}

fn log_impl(ctx: &'static str, rank: i32, size: i32, msg: &str) {
  println!("[{} {}/{}] {}", ctx, rank, size, msg);
}

#[pymodule]
fn neworder(_py: Python, m: &PyModule) -> PyResult<()> {

  #[pyfn(m, "version")]
  fn version(_py: Python) -> PyResult<&str> {
    Ok(env::version())
  }

  // TODO default arg=True
  #[pyfn(m, "verbose")]
  fn verbose(_py: Python, b: bool) -> PyResult<()> {
    Ok(env::verbose(b))
  }

  // TODO default arg=?
  #[pyfn(m, "checked")]
  fn checked(_py: Python, b: bool) -> PyResult<()> {
    Ok(env::checked(b))
  }

  // mpi submodule
  #[pymodule]
  fn mpi(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "rank")]
    fn rank() -> PyResult<i32> {
      Ok(env::rank())
    }

    #[pyfn(m, "size")]
    fn size() -> PyResult<i32> {
      Ok(env::rank())
    }

   Ok(())
  }
  m.add_wrapped(wrap_pymodule!(mpi))?;

  // time submodule
  #[pymodule]
  fn time(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "distant_past")]
    fn distant_past() -> PyResult<f64> {
      Ok(timeline::DISTANT_PAST)
    }

    #[pyfn(m, "far_future")]
    fn far_future() -> PyResult<f64> {
      Ok(timeline::FAR_FUTURE)
    }

    #[pyfn(m, "never")]
    fn never() -> PyResult<f64> {
      Ok(timeline::NEVER)
    }

    #[pyfn(m, "isnever")]
    fn isnever(t: f64) -> PyResult<bool> {
      Ok(timeline::isnever(t))
    }

   Ok(())
  }
  m.add_wrapped(wrap_pymodule!(time))?;


  #[pyfn(m, "log")]
  pub fn log_py(py: Python, x: PyObject) -> PyResult<()> {
    let a = x.as_ref(py);
    log_impl("py", env::rank(), env::size(), &a.str()?.to_string()?);
    Ok(())
  }
  
  m.add_class::<Model>()?;
  m.add_class::<Timeline>()?;
  m.add_class::<MonteCarlo>()?;


  Ok(())
}
