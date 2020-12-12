
use pyo3::prelude::*;
use numpy::PyArray1;
use crate::env;
use crate::montecarlo::MonteCarlo;
use crate::timeline;
use crate::timeline::Timeline;
use crate::model::Model;


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
      Some(b) => env::verbose(b),
      None => env::verbose(true)
    };
    Ok(())
  }

  // default arg=True
  #[pyfn(m, "checked")]
  fn checked(_py: Python, b: Option<bool>) -> PyResult<()> {
    match b {
      Some(b) => env::checked(b),
      None => env::checked(true)
    };
    Ok(())
  }

  #[pyfn(m, "halt")]
  fn halt(_py: Python) -> PyResult<()> {
    env::set_halt(true);
    Ok(())
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

  #[pyfn(df, "unique_index")]
  pub fn unique_index(py: Python, n: usize) -> Py<PyArray1::<i64>> {
    let res = PyArray1::from_vec(py, env::unique_index(n));
    res.to_owned()
  }

  #[pyfn(m, "log")]
  pub fn log_py(py: Python, x: PyObject) -> PyResult<()> {
    let a = x.as_ref(py);
    log_impl("py", env::rank(), env::size(), &a.to_string());
    Ok(())
  }

  m.add_class::<Model>()?;
  m.add_class::<Timeline>()?;
  m.add_class::<MonteCarlo>()?;

  match py.run("import mpi4py.MPI", None, None) {
    Ok(()) => {
      env::set_rank(py.eval("MPI.COMM_WORLD.Get_rank()", None, None)?.extract::<i32>()?);
      env::set_size(py.eval("MPI.COMM_WORLD.Get_size()", None, None)?.extract::<i32>()?);
    },
    Err(_) => {
      // TODO work out whats going wrong here
      // >>> import neworder as no
      // Traceback (most recent call last):
      //   File "<stdin>", line 1, in <module>
      // TypeError: 'tuple' object is not callable
      //PyErr::warn(py, &pyo3::types::PyTuple::empty(py), "mpi4py module not found, assuming serial mode", 0)?;
      env::set_rank(0);
      env::set_size(1);
      log_impl("no", 0, 1, "mpi4py module not found, assuming serial mode");
    }
  }

  Ok(())
}
