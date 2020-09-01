use pyo3::prelude::*;

mod env;
mod model;
use model::Model;
mod timeline;
use timeline::Timeline;

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

  #[pyfn(m, "rank")]
  fn rank(_py: Python) -> PyResult<i32> {
    Ok(env::rank())
  }

  #[pyfn(m, "size")]
  fn size(_py: Python) -> PyResult<i32> {
    Ok(env::rank())
  }

  #[pyfn(m, "log")]
  pub fn log_py(py: Python, x: PyObject) -> PyResult<()> {
    let a = x.as_ref(py);
    log_impl("py", env::rank(), env::size(), &a.str()?.to_string()?);
    Ok(())
  }
  
  pub fn log(msg: &str) {
    log_impl("no", env::rank(), env::size(), msg);
  }
  
  fn log_impl(ctx: &'static str, rank: i32, size: i32, msg: &str) {
    println!("[{} {}/{}] {}", ctx, rank, size, msg);
  }
  

  m.add_class::<Model>()?;
  m.add_class::<Timeline>()?;

  Ok(())
}
